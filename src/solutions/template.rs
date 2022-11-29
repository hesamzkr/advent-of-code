use crate::common::Solution;

pub struct DayX;

impl Solution for DayX {
    type Parsed = Vec<&str>;
    type PartOneOutput = usize;
    type PartTwoOutput = usize;

    fn parse(input: String) -> Self::Parsed {
        input.lines()
    }

    fn part_one(data: Self::Parsed) -> Self::PartOneOutput {}

    fn part_two(data: Self::Parsed) -> Self::PartTwoOutput {}
}

impl DayX {
    fn util(data: <DayX as Solution>::Parsed) {}
}
