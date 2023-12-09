use std::collections::HashMap;

use regex::Regex;

pub fn run(input: String) -> (u64, u64) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let steps: Vec<char> = input[0].chars().collect();

    let re = Regex::new(r"([A-Z1-9]+) = \(([A-Z1-9]+), ([A-Z1-9]+)\)").unwrap();
    let paths: HashMap<String, (String, String)> = input[1]
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            (
                captures[1].to_string(),
                (captures[2].to_string(), captures[3].to_string()),
            )
        })
        .collect();

    // let answer_one = part_one(&steps, &paths);
    let answer_one = 0;
    let answer_two = part_two(&steps, &paths);

    (answer_one, answer_two)
}

fn part_one(steps: &Vec<char>, paths: &HashMap<String, (String, String)>) -> u64 {
    let mut current_step = String::from("AAA");
    let mut num_steps = 0;
    let mut step_index = 0;

    while current_step != "ZZZ" {
        num_steps += 1;

        let (left, right) = paths.get(&current_step).unwrap();
        current_step = match steps[step_index] {
            'L' => left.to_string(),
            'R' => right.to_string(),
            _ => panic!("Invalid step"),
        };
        step_index = (step_index + 1) % steps.len();
    }

    num_steps
}

fn part_two(instructions: &Vec<char>, paths: &HashMap<String, (String, String)>) -> u64 {
    let mut nodes: Vec<String> = paths
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.clone())
        .collect();

    let mut completion_times = vec![];
    for node in nodes.iter_mut() {
        let mut instruction_index = 0;
        let mut steps: u64 = 0;
        while !node.ends_with("Z") {
            let instruction = instructions[instruction_index];
            *node = match instruction {
                'L' => paths.get(node).unwrap().0.clone(),
                'R' => paths.get(node).unwrap().1.clone(),
                _ => panic!("Impossible"),
            };
            steps += 1;
            instruction_index = (instruction_index + 1) % instructions.len();
        }

        completion_times.push(steps);
    }

    let mut complete_time = 1;
    for steps in completion_times {
        complete_time = lcm(complete_time, steps);
    }

    complete_time
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
