use std::collections::HashSet;

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|pair| {
            pair.split(&['-', ','])
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    (part_one, part_two)
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().fold(0, |acc, pair| {
        let first: HashSet<u32> = HashSet::from_iter(pair[0]..=pair[1]);
        let second: HashSet<u32> = HashSet::from_iter(pair[2]..=pair[3]);
        let c = first.intersection(&second).count();

        if c == first.len() || c == second.len() {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().fold(0, |acc, pair| {
        let first: HashSet<u32> = HashSet::from_iter(pair[0]..=pair[1]);
        let second: HashSet<u32> = HashSet::from_iter(pair[2]..=pair[3]);
        let c = first.intersection(&second).count();

        if c > 0 {
            acc + 1
        } else {
            acc
        }
    })
}
