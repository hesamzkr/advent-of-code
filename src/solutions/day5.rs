use std::collections::VecDeque;

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn run(input: String) -> (String, String) {
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

    let part_one = part_one(crane.clone(), &instructions);
    let part_two = part_two(crane.clone(), &instructions);

    (part_one, part_two)
}

fn part_one(mut crane: Vec<VecDeque<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
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

fn part_two(mut crane: Vec<VecDeque<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
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
