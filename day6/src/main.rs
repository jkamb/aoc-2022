use anyhow::Result;
use itertools::Itertools;

type Input = str;

fn find_sequence<const SEQUENCE_COUNT: usize>(input: &Input) -> Result<usize> {
    let res = input
        .as_bytes()
        .windows(SEQUENCE_COUNT)
        .position(is_unique)
        .ok_or(anyhow::anyhow!("No position found"))?;
    Ok(res + SEQUENCE_COUNT)
}

fn is_unique(slice: &[u8]) -> bool {
    let len = slice.len();
    slice.iter().unique().count() == len
}

fn ring_buffer<const COUNT: usize>(input: &Input) -> Result<usize> {
    let mut buffer = [0u8; COUNT];
    buffer.copy_from_slice(&input.as_bytes()[..COUNT]);
    for (index, c) in input.as_bytes().iter().skip(COUNT).enumerate() {
        if !buffer.contains(c) && is_unique(&buffer) {
            return Ok(index + COUNT);
        } else {
            buffer[index % COUNT] = *c;
        }
    }
    Err(anyhow::anyhow!("Failed to find unique position"))
}

fn part1(input: &Input) -> Result<usize> {
    find_sequence::<4>(input)
}

fn part1_ring_buffer(input: &Input) -> Result<usize> {
    ring_buffer::<4>(input)
}
fn part2(input: &Input) -> Result<usize> {
    find_sequence::<14>(input)
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 1 ring buffer: {}", part1_ring_buffer(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input_test.txt");

    #[test]
    fn test_part1() {
        INPUT.lines().for_each(|line| {
            let (input, res, _) = line.split_ascii_whitespace().collect_tuple().unwrap();
            assert_eq!(part1(input).unwrap(), res.parse().unwrap());
        })
    }

    #[test]
    fn test_part1_ring_buffer() {
        INPUT.lines().for_each(|line| {
            let (input, res, _) = line.split_ascii_whitespace().collect_tuple().unwrap();
            assert_eq!(part1_ring_buffer(input).unwrap(), res.parse().unwrap());
        })
    }

    #[test]
    fn test_part2() {
        INPUT.lines().for_each(|line| {
            let (input, _, res) = line.split_ascii_whitespace().collect_tuple().unwrap();
            assert_eq!(part2(input).unwrap(), res.parse().unwrap());
        })
    }
}
