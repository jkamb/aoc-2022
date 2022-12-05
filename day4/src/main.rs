use anyhow::anyhow;

type ElfPair = (Elf, Elf);
type Input = [ElfPair];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    sections: (u32, u32),
}

impl TryFrom<&str> for Elf {
    type Error = anyhow::Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let sections = input
            .split_once('-')
            .map(|(f, s)| {
                let f = f.parse::<u32>().unwrap();
                let s = s.parse::<u32>().unwrap();
                (f, s)
            })
            .ok_or(anyhow!("Failed to parse sections"))?;
        Ok(Self { sections })
    }
}

impl Elf {
    pub fn contains(&self, other: &Elf) -> bool {
        other.sections.0 >= self.sections.0 && other.sections.1 <= self.sections.1
    }

    pub fn overlaps(&self, other: &Elf) -> bool {
        other.sections.0 >= self.sections.0 && other.sections.0 <= self.sections.1
            || other.sections.0 == self.sections.1
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<ElfPair>> {
    input
        .lines()
        .map(|line| line.split_once(',').ok_or(anyhow!("Split failed!")))
        .map(|line| line.map(|(a, b)| (Elf::try_from(a).unwrap(), Elf::try_from(b).unwrap())))
        .collect()
}

fn part1(input: &Input) -> anyhow::Result<u32> {
    let res = input
        .iter()
        .filter(|(elf_a, elf_b)| elf_a.contains(elf_b) || elf_b.contains(elf_a))
        .count();
    Ok(res as u32)
}

fn part2(input: &Input) -> anyhow::Result<u32> {
    let res = input
        .iter()
        .filter(|(elf_a, elf_b)| elf_a.overlaps(elf_b) || elf_b.overlaps(elf_a))
        .count();
    Ok(res as u32)
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
        assert_eq!(res, 2)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 4)
    }
}
