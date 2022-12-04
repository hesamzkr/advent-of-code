use std::collections::HashSet;

use crate::common::Solution;

pub struct Day4;

impl Solution for Day4 {
    type Parsed = Vec<Vec<u32>>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        input
            .lines()
            .map(|pair| {
                pair.split(&['-', ','])
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        data.iter().fold(0, |acc, pair| {
            let first: HashSet<u32> = HashSet::from_iter(pair[0]..=pair[1]);
            let second: HashSet<u32> = HashSet::from_iter(pair[2]..=pair[3]);
            let c = first.intersection(&second).count();

            if c == first.len() || c == second.len() {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        data.iter().fold(0, |acc, pair| {
            let first: HashSet<u32> = HashSet::from_iter(pair[0]..=pair[1]);
            let second: HashSet<u32> = HashSet::from_iter(pair[2]..=pair[3]);
            let c = first.intersection(&second).count();

            if c > 0 {
                acc + 1
            } else {
                acc
            }
        })
    }
}
