use std::{num::ParseIntError, str::FromStr};

use anyhow::anyhow;
use anyhow::Result;

type Calories = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: Calories,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: std::result::Result<Vec<Calories>, ParseIntError> =
            s.lines().map(|line| line.parse()).collect();

        Ok(Elf {
            calories: calories?.into_iter().sum(),
        })
    }
}

type Input = [Elf];

fn parse(input: &str) -> Result<Vec<Elf>> {
    let parsed = input
        .split("\r\n\r\n")
        .map(Elf::from_str)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(parsed)
}

fn part1(input: &Input) -> Result<u32> {
    let result = input.iter().max();
    result
        .map(|elf| elf.calories)
        .ok_or(anyhow!("Failed to find max calorie elf!"))
}

fn part2(input: &Input) -> Result<u32> {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    println!("{}", part1(&input)?);
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
        assert_eq!(res, 5)
    }
}
