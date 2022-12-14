use anyhow::Result;
use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node, Tree,
};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, digit1, line_ending, space1},
    combinator::{eof, map},
    error::context,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

type Input = Tree<Entry>;

type ParseResult<'a, T> = IResult<&'a str, T>;
type ParseInput<'a> = &'a str;

#[derive(Debug)]
enum DirEntry<'a> {
    Dir(&'a str),
    File(usize, &'a str),
}

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<DirEntry<'a>>),
}

fn parse_cd_command(input: ParseInput) -> ParseResult<Command> {
    context(
        "cd command",
        map(
            terminated(
                preceded(tag("$ cd"), tuple((space1, is_not("\r\n")))),
                line_ending,
            ),
            |(_, path)| Command::Cd(path),
        ),
    )(input)
}

fn parse_ls_command(input: ParseInput) -> ParseResult<Command> {
    context(
        "ls command",
        map(
            preceded(
                terminated(tag("$ ls"), line_ending),
                terminated(
                    separated_list1(line_ending, parse_entry),
                    alt((line_ending, eof)),
                ),
            ),
            Command::Ls,
        ),
    )(input)
}

fn parse_dir_entry(input: ParseInput) -> ParseResult<DirEntry<'_>> {
    context(
        "dir_entry",
        map(tuple((tag("dir"), space1, alpha1)), |(_, _, dir)| {
            DirEntry::Dir(dir)
        }),
    )(input)
}

fn parse_file_entry(input: ParseInput) -> ParseResult<DirEntry<'_>> {
    context(
        "file_entry",
        map(
            tuple((digit1, space1, is_not("\r\n"))),
            |(size, _, name)| DirEntry::File(str::parse(size).unwrap(), name),
        ),
    )(input)
}

fn parse_entry(input: ParseInput) -> ParseResult<DirEntry<'_>> {
    context("entry", alt((parse_dir_entry, parse_file_entry)))(input)
}

fn parse_commands(input: ParseInput) -> ParseResult<Vec<Command>> {
    context("command", many1(alt((parse_cd_command, parse_ls_command))))(input)
}
#[derive(Debug, PartialEq)]
enum Entry {
    Dir(String),
    File(String, usize),
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Self::Dir(_) => 0,
            Self::File(_, size) => *size,
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }
}

fn parse(input: &str) -> Result<Tree<Entry>> {
    let (_, commands) = parse_commands(input).unwrap();
    let mut tree = Tree::new();
    let root = tree.insert(Node::new(Entry::Dir("/".to_owned())), AsRoot)?;
    let mut current_dir = root;
    for command in commands {
        match command {
            Command::Cd(path) => {
                if path.starts_with('/') {
                    continue;
                } else if path == ".." {
                    current_dir = tree.get(&current_dir)?.parent().unwrap().clone();
                } else {
                    current_dir = tree.insert(
                        Node::new(Entry::Dir(path.to_owned())),
                        UnderNode(&current_dir),
                    )?;
                }
            }
            Command::Ls(entries) => {
                let entries = entries.into_iter().filter_map(|entry| match entry {
                    DirEntry::Dir(_) => None,
                    DirEntry::File(size, name) => Some(Entry::File(name.to_owned(), size)),
                });
                for entry in entries {
                    tree.insert(Node::new(entry), UnderNode(&current_dir))?;
                }
            }
        }
    }
    Ok(tree)
}

fn calculate_size(tree: &Input, node: &Node<Entry>) -> Result<usize> {
    let mut size = node.data().size();
    for child in node.children() {
        size += calculate_size(tree, tree.get(child)?)?
    }
    Ok(size)
}

fn part1(input: &Input) -> Result<usize> {
    let res = input
        .traverse_pre_order(input.root_node_id().unwrap())?
        .filter(|node| node.data().is_dir())
        .map(|node| calculate_size(input, node).unwrap())
        .filter(|&size| size <= 100_000usize)
        .sum();
    Ok(res)
}

fn part2(input: &Input) -> Result<usize> {
    const MAX_SIZE: usize = 70000000;
    const NEEDED_SIZE: usize = 30000000;

    let root_id = input.root_node_id().unwrap();
    let root_size = calculate_size(input, input.get(root_id)?)?;
    let unused_space = MAX_SIZE - root_size;
    let required_cleanup = NEEDED_SIZE - unused_space;
    input
        .traverse_pre_order(root_id)?
        .filter(|node| node.data().is_dir())
        .map(|node| calculate_size(input, node).unwrap())
        .filter(|&size| size >= required_cleanup)
        .min()
        .ok_or(anyhow::anyhow!("Failed to find min value"))
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input_test.txt");

    #[test]
    fn test_part1() {
        let res = part1(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 95437)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 24933642)
    }
}
