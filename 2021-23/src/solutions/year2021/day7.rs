use std::fs;

pub fn run(input: String) -> (i32, i64) {
    let input: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<i64>) -> i32 {
    let mut lowest_fuel = 1_000_000;
    for pos in 1..2000 {
        let mut fuel = 0;
        for i in input {
            fuel += i32::abs(*i as i32 - pos);
        }

        if fuel < lowest_fuel {
            lowest_fuel = fuel;
        }
    }

    return lowest_fuel;
}

fn part_two(input: &Vec<i64>) -> i64 {
    let mut lowest_fuel: i64 = 1_000_000_000;
    for pos in 1..2000 {
        let mut fuel: i64 = 0;
        for i in input {
            let n = i64::abs(i - pos) as f64;
            let s = (n * 0.5) * (n + 1.0);
            fuel += s as i64;
        }

        if fuel < lowest_fuel {
            lowest_fuel = fuel;
        }
    }

    return lowest_fuel;
}
