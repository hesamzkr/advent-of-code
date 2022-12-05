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
        let mut crane:Vec<VecDeque<char>> = Vec::new();

        for i in 0..9 {
            crane.push(VecDeque::new());
            for j in input.split("\n\n").nth(0).unwrap().lines() {
                let x = j.chars().nth(1 + i * 4).unwrap();
                if x.is_alphabetic() {
                    crane[i].push_front(x);
                }
            }
        }

        let instructions = input
        .split("\n\n")
        .nth(2)
        .unwrap()
        .lines()
        .map(|x| {
            Instruction {
                amount: x.split_whitespace().nth(1).unwrap().parse().unwrap(),
                from: x.split_whitespace().nth(3).unwrap().parse().unwrap(),
                to: x.split_whitespace().nth(5).unwrap().parse().unwrap(),
            }
        }).collect();
        


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
                acc.push(*x)
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
                acc.push(*x)
            }
            acc
        })
    }
}
