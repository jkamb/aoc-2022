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

    fn as_set(&self) -> HashSet<Item> {
        self.items.iter().fold(HashSet::new(), |mut map, item| {
            map.insert(*item);
            map
        })
    }
}

fn parse(input: &str) -> Result<Vec<Rucksack>> {
    let res = input
        .lines()
        .map(|line| Rucksack::new(line.chars().collect()))
        .collect();
    Ok(res)
}

const LOWER_CASE_START: u8 = 1;
const UPPER_CASE_START: u8 = 27;

fn calculate_priority(item: Item) -> u8 {
    let item = item as u8;
    if (b'A'..=b'Z').contains(&item) {
        (item - b'A') + UPPER_CASE_START
    } else {
        (item - b'a') + LOWER_CASE_START
    }
}

fn part1(input: &Input) -> Result<u32> {
    let intersections = input.iter().fold(Vec::new(), |mut vec, item| {
        let first_comp = item.get_first_compartment().as_set();
        let second_comp = item.get_second_compartment().as_set();
        vec.extend(first_comp.intersection(&second_comp));
        vec
    });
    let sum: u8 = intersections
        .iter()
        .map(|item| calculate_priority(*item))
        .sum();
    Ok(sum as u32)
}

fn find_group_badge(rucksacks: &[Rucksack]) -> u32 {
    let badge_iter = rucksacks
        .iter()
        .map(|r| r.as_set())
        .fold(HashSet::new(), |mut badge, item| {
            if badge.is_empty() {
                badge = item.clone();
            }
            badge.intersection(&item).copied().collect()
        })
        .into_iter();
    let badge = Vec::from_iter(badge_iter);
    calculate_priority(*badge.first().unwrap()) as u32
}

fn part2(input: &Input) -> Result<u32> {
    let sum: u32 = input.chunks(3).map(find_group_badge).sum();
    Ok(sum)
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
        assert_eq!(res, 157)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 70)
    }
}
