use crate::common::Solution;

pub struct DayX;

impl Solution for DayX {
    type Parsed = Vec<String>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        todo!()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        0
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        0
    }
}

impl DayX {
    fn util(data: <DayX as Solution>::Parsed) {}
}
