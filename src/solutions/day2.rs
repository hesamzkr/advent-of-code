#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<&str>) -> u32 {
    let mut score = 0;

    let input: Vec<Vec<Move>> = input
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

    for round in input {
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

fn part_two(input: &Vec<&str>) -> u32 {
    let mut score = 0;

    let input: Vec<(Move, Outcome)> = input
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

    for (their_move, outcome) in input {
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
