use crate::common::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Parsed = Vec<u32>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        // input file line endings must be "LF" instead of "CRLF"
        input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|line| line.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        *data.into_iter().max().unwrap()
    }

    fn part_two(mut data: &mut Self::Parsed) -> Self::PartTwoOutput {
        data.sort();
        data.reverse();
        data.iter().take(3).sum()
    }
}
