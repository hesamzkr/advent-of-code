use std::collections::HashSet;

pub fn run(input: String) -> (usize, usize) {
    let input: Vec<char> = input.chars().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<char>) -> usize {
    for i in 0..(input.len() - 3) {
        let mut set: HashSet<char> = HashSet::new();
        set.insert(input[i]);
        set.insert(input[i + 1]);
        set.insert(input[i + 2]);
        set.insert(input[i + 3]);
        if set.len() == 4 {
            return i + 4;
        }
    }
    0
}

fn part_two(input: &Vec<char>) -> usize {
    for i in 0..(input.len() - 13) {
        let mut set: HashSet<char> = HashSet::new();
        for j in 0..14 {
            set.insert(input[i + j]);
        }
        if set.len() == 14 {
            return i + 14;
        }
    }
    0
}
