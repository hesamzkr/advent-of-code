pub fn run(input: String) -> (u32, u32) {
    let mut input: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();

    let part_one = part_one(&input);
    let part_two = part_two(&mut input);

    (part_one, part_two)
}

fn part_one(input: &Vec<u32>) -> u32 {
    *input.iter().max().unwrap()
}

fn part_two(input: &mut Vec<u32>) -> u32 {
    input.sort();
    input.reverse();
    input.iter().take(3).sum()
}
