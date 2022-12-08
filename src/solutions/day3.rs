use std::collections::HashSet;

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for sack in input {
        let mid = sack.len() / 2;
        let first: HashSet<char> = HashSet::from_iter(sack.chars().take(mid));
        let second: HashSet<char> = HashSet::from_iter(sack.chars().skip(mid));

        sum += first
            .intersection(&second)
            .fold(0, |acc, c| acc + char_to_num(*c));
    }
    sum
}

fn part_two(input: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for i in (0..=input.len() - 3).step_by(3) {
        let r1: HashSet<char> = HashSet::from_iter(input[i].chars());
        let r2: HashSet<char> = HashSet::from_iter(input[i + 1].chars());
        let r3: HashSet<char> = HashSet::from_iter(input[i + 2].chars());

        sum += r1
            .intersection(&r2)
            .map(|x| *x)
            .collect::<HashSet<char>>()
            .intersection(&r3)
            .fold(0, |acc, c| acc + char_to_num(*c));
    }
    sum
}

fn char_to_num(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}
