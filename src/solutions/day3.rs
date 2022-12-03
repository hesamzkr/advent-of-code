use std::collections::HashSet;

use crate::common::Solution;

pub struct Day3;

impl Solution for Day3 {
    type Parsed = Vec<String>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.to_string()).collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut sum = 0;
        for sack in data {
            let mid = sack.len() / 2;
            let mut first: HashSet<char> = HashSet::from_iter(sack.chars().take(mid));
            let mut second: HashSet<char> = HashSet::from_iter(sack.chars().skip(mid));

            sum += first
                .intersection(&second)
                .fold(0, |acc, c| acc + Self::char_to_num(*c));
        }
        sum
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut sum = 0;
        for i in (0..=data.len() - 3).step_by(3) {
            let r1: HashSet<char> = HashSet::from_iter(data[i].chars());
            let r2: HashSet<char> = HashSet::from_iter(data[i + 1].chars());
            let r3: HashSet<char> = HashSet::from_iter(data[i + 2].chars());

            sum += r1
                .intersection(&r2)
                .map(|x| *x)
                .collect::<HashSet<char>>()
                .intersection(&r3)
                .fold(0, |acc, c| acc + Self::char_to_num(*c));
        }
        sum
    }
}

impl Day3 {
    fn char_to_num(c: char) -> u32 {
        if c.is_lowercase() {
            c as u32 - 'a' as u32 + 1
        } else {
            c as u32 - 'A' as u32 + 27
        }
    }
}
