use anyhow::Result;

type Input = [u32];

fn parse(input: &str) -> Result<Vec<u32>> {
    todo!()
}

fn part1(input: &Input) -> Result<u32> {
    todo!()
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
        assert_eq!(res, 7)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 5)
    }
}
