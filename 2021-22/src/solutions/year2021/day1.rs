pub fn run(input: String) -> (u32, u32) {
    let input: Vec<u32> = input
    .lines()
    .map(|x| x.parse().unwrap())
    .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &[u32]) -> u32 {
    let mut previous = input[0];
    let mut increased = 0;

    for i in 1..input.len() {
        if input[i] > previous {
            increased += 1;
        }
        previous = input[i];
    }

    increased
}

fn part_two(input: &[u32]) -> u32 {
    let mut previous = input[0] + input[1] + input[2];
    let mut increased = 0;
    for i in 1..input.len() - 2 {
        let new = input[i] + input[i + 1] + input[i + 2];
        if new > previous {
            increased += 1;
        }

        previous = new;

    }

    increased
}
