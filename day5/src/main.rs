use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, anychar, digit1, line_ending},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

type Input = (Stacks, Instructions);

#[derive(Debug, Clone, Copy)]
struct Crate(char);

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
    fn from_rows(rows: Vec<Vec<Option<Crate>>>) -> Result<Self> {
        if rows.is_empty() {
            return Err(anyhow::anyhow!("No rows to be found!"));
        }

        let len = rows[0].len();
        let mut iter: Vec<_> = rows.into_iter().map(|inner| inner.into_iter()).collect();
        let stacks = (0..len)
            .map(|_| {
                iter.iter_mut()
                    .rev()
                    .filter_map(|inner| inner.next())
                    .flatten()
                    .collect()
            })
            .collect();
        Ok(Self(stacks))
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub amount: u32,
    pub from: u32,
    pub to: u32,
}

type Instructions = Vec<Instruction>;

type ParseResult<'a, T> = IResult<&'a str, T>;
type ParseInput<'a> = &'a str;

fn some_crate(input: ParseInput) -> ParseResult<Option<Crate>> {
    let p = delimited(complete::char('['), anychar, complete::char(']'));
    map(p, |c| Some(Crate(c)))(input)
}

fn none_crate(input: ParseInput) -> ParseResult<Option<Crate>> {
    value(None, tag("   "))(input)
}

fn parse_stacks(input: ParseInput) -> ParseResult<Vec<Vec<Option<Crate>>>> {
    let crate_rows = separated_list1(complete::char(' '), alt((some_crate, none_crate)));
    let all_rows = separated_list1(line_ending, crate_rows);
    terminated(all_rows, complete::char('\n'))(input)
}

fn skip_line(input: ParseInput) -> ParseResult<()> {
    value((), terminated(take_until("\n"), line_ending))(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_instructions(input: ParseInput) -> ParseResult<Instructions> {
    let instructions = map(
        tuple((
            tag("move "),
            parse_u32,
            tag(" from "),
            parse_u32,
            tag(" to "),
            parse_u32,
        )),
        |(_, amount, _, from, _, to)| Instruction {
            amount,
            from: from - 1,
            to: to - 1,
        },
    );
    separated_list1(line_ending, instructions)(input)
}

fn parse(input: &str) -> Result<Input> {
    let (remaining, rows) = parse_stacks(input).unwrap();
    let stacks = Stacks::from_rows(rows)?;

    let (remaining, _) = skip_line(remaining).unwrap();
    let (remaining, _) = skip_line(remaining).unwrap();
    let (_, instructions) = parse_instructions(remaining).unwrap();
    Ok((stacks, instructions))
}

fn part1(input: Input) -> Result<String> {
    let (mut stacks, instructions) = input;
    for inst in instructions {
        for _ in 0..inst.amount {
            let src = stacks.0[inst.from as usize]
                .pop()
                .ok_or(anyhow::anyhow!("Failed to pop from source stack!"))?;
            stacks.0[inst.to as usize].push(src);
        }
    }
    let res: String = stacks
        .0
        .into_iter()
        .map(|stack| stack.last().map(|c| c.0).unwrap())
        .collect();

    Ok(res)
}

fn part2(input: Input) -> Result<String> {
    let (mut stacks, instructions) = input;
    for inst in instructions {
        let from = inst.from;
        let to = inst.to;
        let (src, dst) = if from < to {
            let (l, r) = stacks.0.split_at_mut(to as usize);
            (&mut l[from as usize], &mut r[0])
        } else {
            let (l, r) = stacks.0.split_at_mut(from as usize);
            (&mut r[0], &mut l[to as usize])
        };
        let drain_amount = src.len() - (inst.amount as usize);
        dst.extend(src.drain(drain_amount..));
    }
    let res: String = stacks
        .0
        .into_iter()
        .map(|stack| stack.last().map(|c| c.0).unwrap())
        .collect();

    Ok(res)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    println!("Part 1: {}", part1(input.clone())?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input_test.txt");

    #[test]
    fn test_part1() {
        let res = part1(parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, String::from("CMZ"))
    }

    #[test]
    fn test_part2() {
        let res = part2(parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, String::from("MCD"))
    }
}
