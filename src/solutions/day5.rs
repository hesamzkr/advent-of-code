use std::collections::VecDeque;

use crate::common::Solution;

pub struct Day5;

pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Solution for Day5 {
    type Parsed = (Vec<VecDeque<char>>, Vec<Instruction>);
    type PartOneOutput = String;
    type PartTwoOutput = String;

    fn parse(input: String) -> Self::Parsed {
        let input: Vec<&str> = input.split("\n\n").collect();
        let mut crane: Vec<VecDeque<char>> = Vec::new();

        for i in 0..9 {
            crane.push(VecDeque::new());
            for j in input[0].lines() {
                let x = j.chars().nth(1 + i * 4).unwrap();
                if x.is_alphabetic() {
                    crane[i].push_front(x);
                }
            }
        }

        let instructions = input[2]
            .lines()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .map(|x| Instruction {
                amount: x[1].parse().unwrap(),
                from: x[3].parse().unwrap(),
                to: x[5].parse().unwrap(),
            })
            .collect();

        (crane, instructions)
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut crane = data.0.clone();

        for instruction in &data.1 {
            for i in 0..instruction.amount {
                if let Some(x) = crane[instruction.from - 1].pop_back() {
                    crane[instruction.to - 1].push_back(x);
                }
            }
        }

        crane.iter().fold(String::new(), |mut acc, i| {
            if let Some(x) = i.get(i.len() - 1) {
                acc.push(*x);
            }
            acc
        })
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut crane = data.0.clone();

        for instruction in &data.1 {
            let mut temp: VecDeque<char> = VecDeque::new();
            for i in 0..instruction.amount {
                if let Some(x) = crane[instruction.from - 1].pop_back() {
                    temp.push_front(x);
                }
            }
            for i in temp {
                crane[instruction.to - 1].push_back(i);
            }
        }

        crane.iter().fold(String::new(), |mut acc, i| {
            if let Some(x) = i.get(i.len() - 1) {
                acc.push(*x);
            }
            acc
        })
    }
}
