use std::{collections::HashMap, vec};

enum Instruction {
    Move(i32),
    Rotation(char),
}

pub fn run(input: String) -> (i32, u32) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input[0].lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != ' ' {
                map.insert(((x + 1) as i32, (y + 1) as i32), char);
            }
        }
    }

    let mut chars = input[1].chars().peekable();
    let mut path: Vec<Instruction> = Vec::new();

    while chars.peek().is_some() {
        let char = chars.next().unwrap();
        if char.is_numeric() {
            if let Some(next_char) = chars.peek() {
                if next_char.is_numeric() {
                    path.push(Instruction::Move(
                        format!("{}{}", char, next_char).parse().unwrap(),
                    ));
                    chars.next();
                    continue;
                }
            }

            path.push(Instruction::Move(char.to_digit(10).unwrap() as i32));
        } else {
            path.push(Instruction::Rotation(char));
        }
    }

    let answer_one = part_one(&map, &path);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(map: &HashMap<(i32, i32), char>, path: &Vec<Instruction>) -> i32 {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut position = (get_row(map, 1).min().unwrap(), 1);
    let mut facing = 0;

    for instruction in path {
        match instruction {
            Instruction::Move(length) => {
                for _ in 0..*length {
                    let dir = directions[facing];
                    let new_pos = (position.0 + dir.0, position.1 + dir.1);
                    match map.get(&new_pos) {
                        Some(c) => {
                            if c == &'.' {
                                position = new_pos;
                            }
                        }
                        None => {
                            let new_pos = match facing {
                                0 => (get_row(map, position.1).min().unwrap(), position.1),
                                1 => (position.0, get_col(map, position.0).min().unwrap()),
                                2 => (get_row(map, position.1).max().unwrap(), position.1),
                                3 => (position.0, get_col(map, position.0).max().unwrap()),
                                _ => panic!("Invalid direction"),
                            };

                            if map.get(&new_pos).unwrap() == &'.' {
                                position = new_pos;
                            }
                        }
                    }
                }
            }
            Instruction::Rotation(char) => {
                let dir = if char == &'L' { -1 } else { 1 };
                facing = (facing as isize + dir).rem_euclid(4) as usize;
            }
        }
    }

    1000 * position.1 + 4 * position.0 + facing as i32
}

fn part_two(input: &[&str]) -> u32 {
    0
}

fn get_row<'h>(map: &'h HashMap<(i32, i32), char>, target: i32) -> impl 'h + Iterator<Item = i32> {
    map.iter()
        .filter_map(move |((x, y), tile)| (*y == target).then_some(*x))
}

fn get_col<'h>(map: &'h HashMap<(i32, i32), char>, target: i32) -> impl 'h + Iterator<Item = i32> {
    map.iter()
        .filter_map(move |((x, y), tile)| (*x == target).then_some(*y))
}
