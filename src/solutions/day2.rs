use crate::common::Solution;

pub struct Day2;

#[derive(Clone, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Solution for Day2 {
    type Parsed = Vec<Vec<Shape>>;
    type PartOneOutput = u64;
    type PartTwoOutput = u64;

    fn parse(input: String) -> Self::Parsed {
        input
            .lines()
            .map(|x| {
                x.split_whitespace()
                    .map(|choice| {
                        if choice == "A" || choice == "X" {
                            Shape::Rock
                        } else if choice == "B" || choice == "Y" {
                            Shape::Paper
                        } else {
                            Shape::Scissors
                        }
                    })
                    .collect::<Vec<Shape>>()
            })
            .collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut total = 0;
        for round in data {
            let (their_move, your_move) = (&round[0], &round[1]);

            total += match your_move {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };

            total += match (their_move, your_move) {
                _ if their_move == your_move => 3,
                (Shape::Rock, Shape::Paper) => 6,
                (Shape::Paper, Shape::Scissors) => 6,
                (Shape::Scissors, Shape::Rock) => 6,
                _ => 0,
            };
        }

        total
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut total = 0;
        for round in data {
            let (their_move, outcome) = (&round[0], &round[1]);

            let (your_move, outcome_score) = match (their_move, outcome) {
                (Shape::Rock, Shape::Rock) => (Shape::Scissors, 0),
                (Shape::Paper, Shape::Rock) => (Shape::Rock, 0),
                (Shape::Scissors, Shape::Rock) => (Shape::Paper, 0),

                (Shape::Rock, Shape::Paper) => (Shape::Rock, 3),
                (Shape::Paper, Shape::Paper) => (Shape::Paper, 3),
                (Shape::Scissors, Shape::Paper) => (Shape::Scissors, 3),

                (Shape::Rock, Shape::Scissors) => (Shape::Paper, 6),
                (Shape::Paper, Shape::Scissors) => (Shape::Scissors, 6),
                (Shape::Scissors, Shape::Scissors) => (Shape::Rock, 6),
            };

            total += match your_move {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };

            total += outcome_score;
        }

        total
    }
}
