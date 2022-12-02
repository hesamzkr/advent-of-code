use crate::common::Solution;

pub struct Day2;

#[derive(PartialEq, Clone, Copy)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Solution for Day2 {
    type Parsed = Vec<String>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.to_string()).collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut score = 0;

        let data: Vec<Vec<Move>> = data
            .iter()
            .map(|game| {
                game.split_whitespace()
                    .map(|char| match char {
                        "A" => Move::Rock,
                        "B" => Move::Paper,
                        "C" => Move::Scissors,
                        "X" => Move::Rock,
                        "Y" => Move::Paper,
                        "Z" => Move::Scissors,
                        _ => panic!("invalid character"),
                    })
                    .collect()
            })
            .collect();

        for round in data {
            let (their_move, your_move) = (&round[0], &round[1]);

            score += *your_move as u32;

            score += match (their_move, your_move) {
                _ if their_move == your_move => Outcome::Draw,
                (Move::Rock, Move::Paper) => Outcome::Win,
                (Move::Paper, Move::Scissors) => Outcome::Win,
                (Move::Scissors, Move::Rock) => Outcome::Win,
                _ => Outcome::Loss,
            } as u32;
        }

        score
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut score = 0;

        let data: Vec<(Move, Outcome)> = data
            .iter()
            .map(|game| {
                let their_move = match game.chars().nth(0).unwrap() {
                    'A' => Move::Rock,
                    'B' => Move::Paper,
                    'C' => Move::Scissors,
                    _ => panic!("Invalid move"),
                };

                let outcome = match game.chars().nth(2).unwrap() {
                    'X' => Outcome::Loss,
                    'Y' => Outcome::Draw,
                    'Z' => Outcome::Win,
                    _ => panic!("Invalid outcome"),
                };

                (their_move, outcome)
            })
            .collect();

        for (their_move, outcome) in data {
            score += match &outcome {
                Outcome::Win => match their_move {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                },
                Outcome::Draw => their_move,
                Outcome::Loss => match their_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                },
            } as u32;

            score += outcome as u32;
        }
        score
    }
}
