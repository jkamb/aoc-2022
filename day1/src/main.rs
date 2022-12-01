use anyhow::anyhow;
use std::{num::ParseIntError, str::FromStr};

type Calories = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: Calories,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: Result<Vec<Calories>, ParseIntError> =
            s.lines().map(|line| line.parse()).collect();

        Ok(Elf {
            calories: calories?.into_iter().sum(),
        })
    }
}

type Input = [Elf];

fn parse(input: &str) -> anyhow::Result<Vec<Elf>> {
    let mut parsed = input
        .split("\r\n\r\n")
        .map(Elf::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort();
    Ok(parsed)
}

fn part1(input: &Input) -> anyhow::Result<u32> {
    let result = input.iter().max();
    result
        .map(|elf| elf.calories)
        .ok_or(anyhow!("Failed to find max calorie elf!"))
}

fn part2(input: &Input) -> anyhow::Result<u32> {
    let result = input
        .iter()
        .rev()
        .take(3)
        .map(|elf| elf.calories)
        .sum::<Calories>();
    Ok(result)
}

fn main() -> anyhow::Result<()> {
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
        assert_eq!(res, 24000)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 45000)
    }
}
