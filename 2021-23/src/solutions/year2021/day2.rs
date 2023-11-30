use crate::point;

enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

pub fn run(input: String) -> (i64, i64) {
    let input: Vec<Command> = input
    .lines()
    .map(|line| line.split_whitespace())
    .map(|mut split| match split.next().unwrap() {
        "forward" => Command::Forward(split.next().unwrap().parse().unwrap()),
        "up" => Command::Up(split.next().unwrap().parse().unwrap()),
        "down" => Command::Down(split.next().unwrap().parse().unwrap()),
        _ => panic!("Invalid command"),
    }).collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(commands: &[Command]) -> i64 {
    let mut position = point!(0, 0);

    for command in commands {
        match command {
            Command::Forward(arg) => position.x += arg,
            Command::Up(arg) => position.y -= arg,
            Command::Down(arg) => position.y += arg, 
        }
    }

    position.x * position.y
}

fn part_two(commands: &[Command]) -> i64 {
    let mut position = point!(0, 0);
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(arg) => {
                position.x += arg;
                position.y += aim * arg;
            },
            Command::Up(arg) => aim -= arg,
            Command::Down(arg) => aim += arg, 
        }
    }

    position.x * position.y
}