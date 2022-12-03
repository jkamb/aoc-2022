use std::collections::HashSet;

use anyhow::Result;

type Input = [Rucksack];
type Item = char;

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

#[derive(Debug)]
struct Compartment<'a> {
    items: &'a [Item],
}

impl<'a> Compartment<'a> {
    fn as_set(&self) -> HashSet<Item> {
        self.items.iter().fold(HashSet::new(), |mut map, item| {
            map.insert(*item);
            map
        })
    }
}

impl Rucksack {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    fn get_first_compartment(&self) -> Compartment<'_> {
        let len = self.items.len() / 2;
        Compartment {
            items: &self.items[..len],
        }
    }

    fn get_second_compartment(&self) -> Compartment<'_> {
        let len = self.items.len() / 2;
        Compartment {
            items: &self.items[len..],
        }
    }
}

fn parse(input: &str) -> Result<Vec<Rucksack>> {
    let res = input
        .lines()
        .map(|line| Rucksack::new(line.chars().collect()))
        .collect();
    Ok(res)
}

const LOWER_CASE_START: u32 = 1;
const LOWER_CASE_ASCII_START: u32 = 97;

const UPPER_CASE_START: u32 = 27;
const UPPER_CASE_ASCII_START: u32 = 65;

fn calculate_priority(item: Item) -> u32 {
    if ('A'..='Z').contains(&item) {
        item as u32 - UPPER_CASE_ASCII_START + UPPER_CASE_START
    } else {
        item as u32 - LOWER_CASE_ASCII_START + LOWER_CASE_START
    }
}

fn part1(input: &Input) -> Result<u32> {
    let intersections = input.iter().fold(Vec::new(), |mut vec, item| {
        let first_comp = item.get_first_compartment().as_set();
        let second_comp = item.get_second_compartment().as_set();
        vec.extend(first_comp.intersection(&second_comp));
        vec
    });
    let sum = intersections
        .iter()
        .map(|item| calculate_priority(*item))
        .sum();
    Ok(sum)
}

fn part2(input: &Input) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    println!("Part 1: {}", part1(&input)?);
    //println!("Part 2: {}", part1(&input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input_test.txt");

    #[test]
    fn test_part1() {
        let res = part1(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 157)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 5)
    }
}
