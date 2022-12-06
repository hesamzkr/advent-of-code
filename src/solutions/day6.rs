use std::collections::HashSet;

use crate::common::Solution;

pub struct Day6;

impl Solution for Day6 {
    type Parsed = Vec<char>;
    type PartOneOutput = usize;
    type PartTwoOutput = usize;

    fn parse(input: String) -> Self::Parsed {
        input.chars().collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        for i in 0..data.len() {
            let mut set: HashSet<char> = HashSet::new();
            set.insert(data[i]);
            set.insert(data[i + 1]);
            set.insert(data[i + 2]);
            set.insert(data[i + 3]);
            if set.len() == 4 {
                return i + 4;
            }
        }
        0
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        for i in 0..data.len() {
            let mut set: HashSet<char> = HashSet::new();
            for j in 0..14 {
                set.insert(data[i + j]);
            }
            if set.len() == 14 {
                return i + 14;
            }
        }
        0
    }
}
