use regex::{Match, Regex};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum Part {
    Number((u32, isize)), // (number, length)
    Symbol(char),
}

type Point = (isize, isize); // (row, col)

pub fn run(input: String) -> (u32, u32) {
    let pattern = Regex::new(r"\d+").unwrap();
    let input: HashMap<Point, Part> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let matches = pattern.find_iter(line).map(move |m| {
                let number = m.as_str().parse::<u32>().unwrap();
                (
                    (i as isize, m.start() as isize),
                    Part::Number((number, m.as_str().len() as isize)),
                )
            });

            let symbols = line.chars().enumerate().filter_map(move |(j, char)| {
                if char != '.' && !char.is_numeric() {
                    Some(((i as isize, j as isize), Part::Symbol(char)))
                } else {
                    None
                }
            });

            matches.chain(symbols)
        })
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(schematic: &HashMap<Point, Part>) -> u32 {
    let mut sum = 0;

    for ((row, col), part) in schematic {
        match part {
            Part::Symbol(_) => continue,
            Part::Number((number, length)) => {
                let mut is_valid = false;

                for x in (*col..(col + length)) {
                    let checks = [
                        (row - 1, x - 1),
                        (row - 1, x),
                        (row - 1, x + 1),
                        (*row, x - 1),
                        (*row, x + 1),
                        (row + 1, x - 1),
                        (row + 1, x),
                        (row + 1, x + 1),
                    ];

                    for check in checks {
                        if let Some(other) = schematic.get(&check) {
                            match other {
                                Part::Symbol(_) => is_valid = true,
                                Part::Number(_) => continue,
                            }
                        }
                    }
                }

                if is_valid {
                    sum += number;
                }
            }
        }
    }

    sum
}

fn part_two(schematic: &HashMap<Point, Part>) -> u32 {
    let mut sum = 0;
    let mut gear_numbers: Vec<(&Part, Point)> = vec![];

    for ((row, col), part) in schematic {
        match part {
            Part::Symbol(_) => continue,
            Part::Number((number, length)) => {
                let mut gear_index: Option<Point> = None;

                for x in (*col..(col + length)) {
                    let checks = [
                        (row - 1, x - 1),
                        (row - 1, x),
                        (row - 1, x + 1),
                        (*row, x - 1),
                        (*row, x + 1),
                        (row + 1, x - 1),
                        (row + 1, x),
                        (row + 1, x + 1),
                    ];

                    for check in checks {
                        if let Some(other) = schematic.get(&check) {
                            match other {
                                Part::Symbol('*') => gear_index = Some(check),
                                Part::Number(_) | Part::Symbol(_) => continue,
                            }
                        }
                    }
                }

                if let Some(point) = gear_index {
                    gear_numbers.push((part, point));
                }
            }
        }
    }

    for i in 0..gear_numbers.len() - 1 {
        for j in (i + 1)..gear_numbers.len() {
            if let (Part::Number((num_1, _)), point) = gear_numbers[i] {
                if let (Part::Number((num_2, _)), other_point) = gear_numbers[j] {
                    if point == other_point {
                        sum += num_1 * num_2;
                    }
                }
            }
        }
    }

    sum
}
