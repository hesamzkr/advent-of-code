use crate::lib::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Parsed = Vec<u32>;
    type PartOneOutput = usize;
    type PartTwoOutput = usize;

    fn parse(input: String) -> Self::Parsed {
        vec![2]
    }

    fn part_one(data: Self::Parsed) -> Self::PartOneOutput {
        5
    }

    fn part_two(data: Self::Parsed) -> Self::PartTwoOutput {
        5
    }
}
