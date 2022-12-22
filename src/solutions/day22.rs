use std::collections::HashMap;

enum Instruction {
    Move(i32),
    Rotate(char),
}

// [right, down, left, up]
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn run(input: String) -> (i32, i32) {
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
    let mut instructions: Vec<Instruction> = Vec::new();

    while chars.peek().is_some() {
        let char = chars.next().unwrap();
        if char.is_numeric() {
            if let Some(next_char) = chars.peek() {
                if next_char.is_numeric() {
                    instructions.push(Instruction::Move(
                        format!("{}{}", char, next_char).parse().unwrap(),
                    ));
                    chars.next();
                    continue;
                }
            }

            instructions.push(Instruction::Move(char.to_digit(10).unwrap() as i32));
        } else {
            instructions.push(Instruction::Rotate(char));
        }
    }

    let answer_one = part_one(&map, &instructions);
    let answer_two = part_two(&map, &instructions);

    (answer_one, answer_two)
}

fn part_one(map: &HashMap<(i32, i32), char>, instructions: &Vec<Instruction>) -> i32 {
    let mut position = (get_row(map, 1).min().unwrap(), 1);
    let mut facing = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Move(length) => {
                for _ in 0..*length {
                    let dir = DIRECTIONS[facing];
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
            Instruction::Rotate(char) => {
                let dir = if char == &'L' { -1 } else { 1 };
                facing = (facing as isize + dir).rem_euclid(4) as usize;
            }
        }
    }

    1000 * position.1 + 4 * position.0 + facing as i32
}

fn part_two(map: &HashMap<(i32, i32), char>, instructions: &Vec<Instruction>) -> i32 {
    let mut position = (get_row(map, 1).min().unwrap(), 1);
    let mut facing = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Move(length) => {
                for _ in 0..*length {
                    let dir = DIRECTIONS[facing];
                    let (x, y) = position;
                    let (mut next_x, mut next_y) = (position.0 + dir.0, position.1 + dir.1);
                    let mut next_facing = facing;

                    // wrap right
                    if next_x == 151 && (1..=50).contains(&y) {
                        next_x = 100;
                        next_y = 151 - y;
                        next_facing = 2;
                    } else if next_x == 101 && (51..=100).contains(&y) {
                        next_x = y + 50;
                        next_y = 50;
                        next_facing = 3;
                    } else if next_x == 101 && (101..=150).contains(&y) {
                        next_x = 150;
                        next_y = 151 - y;
                        next_facing = 2;
                    } else if next_x == 51 && (151..=200).contains(&y) {
                        next_x = y - 100;
                        next_y = 150;
                        next_facing = 3;
                    }
                    // wrap left
                    else if next_x == 50 && (1..=50).contains(&y) {
                        next_x = 1;
                        next_y = 151 - y;
                        next_facing = 0;
                    } else if next_x == 50 && (51..=100).contains(&y) {
                        next_x = y - 50;
                        next_y = 101;
                        next_facing = 1;
                    } else if next_x == 0 && (101..=150).contains(&y) {
                        next_x = 51;
                        next_y = 151 - y;
                        next_facing = 0;
                    } else if next_x == 0 && (151..=200).contains(&y) {
                        next_x = y - 100;
                        next_y = 1;
                        next_facing = 1;
                    }
                    // wrap up
                    else if next_y == 100 && (1..=50).contains(&x) {
                        next_x = 51;
                        next_y = x + 50;
                        next_facing = 0;
                    } else if next_y == 0 && (51..=100).contains(&x) {
                        next_x = 1;
                        next_y = x + 100;
                        next_facing = 0;
                    } else if next_y == 0 && (101..=150).contains(&x) {
                        next_x = x - 100;
                        next_y = 200;
                        next_facing = 3;
                    }
                    // wrap down
                    else if next_y == 201 && (1..=50).contains(&x) {
                        next_x = x + 100;
                        next_y = 1;
                        next_facing = 1;
                    } else if next_y == 151 && (51..=100).contains(&x) {
                        next_x = 50;
                        next_y = x + 100;
                        next_facing = 2;
                    } else if next_y == 51 && (101..=150).contains(&x) {
                        next_x = 100;
                        next_y = x - 50;
                        next_facing = 2;
                    }

                    if map.get(&(next_x, next_y)).unwrap() == &'.' {
                        position = (next_x, next_y);
                        facing = next_facing;
                    }
                }
            }
            Instruction::Rotate(char) => {
                let dir = if char == &'L' { -1 } else { 1 };
                facing = (facing as isize + dir).rem_euclid(4) as usize;
            }
        }
    }

    1000 * position.1 + 4 * position.0 + facing as i32
}

fn get_row(map: &'_ HashMap<(i32, i32), char>, target: i32) -> impl '_ + Iterator<Item = i32> {
    map.iter()
        .filter_map(move |((x, y), _)| (*y == target).then_some(*x))
}

fn get_col(map: &'_ HashMap<(i32, i32), char>, target: i32) -> impl '_ + Iterator<Item = i32> {
    map.iter()
        .filter_map(move |((x, y), _)| (*x == target).then_some(*y))
}
