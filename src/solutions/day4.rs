use std::collections::HashSet;

use crate::common::Solution;

pub struct Day4;

impl Solution for Day4 {
    type Parsed = Vec<Vec<String>>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.split(",").map(|y| y.to_string()).collect()).collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut total = 0;
        for pair in data {
            let x: Vec<u32> = pair[0].split("-").map(|y| y.parse().unwrap()).collect();
            let y: Vec<u32> = pair[1].split("-").map(|y| y.parse().unwrap()).collect();

            let first: HashSet<u32> = HashSet::from_iter((x[0]..=x[1]));
            let second: HashSet<u32> = HashSet::from_iter((y[0]..=y[1]));

            let c = first.intersection(&second).count();

            if c == first.len() || c == second.len() {
                total += 1;
            }
        }



        total
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut total = 0;
        for pair in data {
            let x: Vec<u32> = pair[0].split("-").map(|y| y.parse().unwrap()).collect();
            let y: Vec<u32> = pair[1].split("-").map(|y| y.parse().unwrap()).collect();

            let first: HashSet<u32> = HashSet::from_iter((x[0]..=x[1]));
            let second: HashSet<u32> = HashSet::from_iter((y[0]..=y[1]));

            let c = first.intersection(&second).count();

            if c > 0 {
                total += 1;
            }
        }
        total
    }
}

impl Day4 {
    fn util(data: <Day4 as Solution>::Parsed) {}
}
