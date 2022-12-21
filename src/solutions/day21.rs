use std::collections::HashMap;

enum Operation {
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
    Num(i64),
}

pub fn run(input: String) -> (i64, u32) {
    let monkeys: HashMap<String, Operation> = input
        .lines()
        .map(|line| {
            let operation = match line.split(": ").nth(1).unwrap().parse::<i64>() {
                Ok(x) => Operation::Num(x),
                Err(_) => {
                    let split: Vec<String> = line[6..]
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect();
                    match split[1].as_str() {
                        "+" => Operation::Add(split[0].clone(), split[2].clone()),
                        "-" => Operation::Sub(split[0].clone(), split[2].clone()),
                        "*" => Operation::Mult(split[0].clone(), split[2].clone()),
                        "/" => Operation::Div(split[0].clone(), split[2].clone()),
                        _ => panic!("Invalid operator"),
                    }
                }
            };
            (line[0..4].to_owned(), operation)
        })
        .collect();

    let answer_one = part_one(&monkeys, &String::from("root"));
    let answer_two = part_two(&monkeys);

    (answer_one, answer_two)
}

fn part_one(monkeys: &HashMap<String, Operation>, target: &String) -> i64 {
    match monkeys.get(target).unwrap() {
        Operation::Add(a, b) => part_one(monkeys, a) + part_one(monkeys, b),
        Operation::Sub(a, b) => part_one(monkeys, a) - part_one(monkeys, b),
        Operation::Mult(a, b) => part_one(monkeys, a) * part_one(monkeys, b),
        Operation::Div(a, b) => part_one(monkeys, a) / part_one(monkeys, b),
        Operation::Num(x) => *x,
    }
}

fn part_two(monkeys: &HashMap<String, Operation>) -> u32 {
    0
}
