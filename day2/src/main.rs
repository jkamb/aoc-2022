use std::{char::ParseCharError, str::FromStr};

use anyhow::anyhow;
use itertools::Itertools;

type Input = [ShapeTuple];
type Score = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl FromStr for Shape {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => todo!(),
        };
        Ok(shape)
    }
}

type ShapeTuple = (Shape, Shape);

fn parse_shape_tuple(input: &str) -> anyhow::Result<ShapeTuple> {
    let (theirs, ours) = input
        .split(' ')
        .collect_tuple()
        .ok_or(anyhow!("Failed to collect"))?;
    let parsed_tuple = (Shape::from_str(theirs)?, Shape::from_str(ours)?);
    Ok(parsed_tuple)
}

enum Round {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

fn parse(input: &str) -> anyhow::Result<Vec<ShapeTuple>> {
    input.lines().map(parse_shape_tuple).collect()
}

fn calculate_score(shapes: &ShapeTuple) -> Score {
    let (theirs, ours) = shapes;
    dbg!(&shapes);

    if theirs == ours {
        return Round::Draw as u32 + *ours as u32;
    }

    match (ours, theirs) {
        (Shape::Paper, Shape::Rock) => Round::Win as u32 + *ours as u32,
        (Shape::Rock, Shape::Scissor) => Round::Win as u32 + *ours as u32,
        (Shape::Scissor, Shape::Paper) => Round::Win as u32 + *ours as u32,
        _ => Round::Lost as u32 + *ours as u32,
    }
}

fn part1(input: &Input) -> anyhow::Result<Score> {
    let score = input.iter().map(calculate_score).sum();
    Ok(score)
}

fn part2(input: &Input) -> anyhow::Result<u32> {
    todo!()
}

fn main() -> anyhow::Result<()> {
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
        assert_eq!(res, 15)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 12)
    }
}
