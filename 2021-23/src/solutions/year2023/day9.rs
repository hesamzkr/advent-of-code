pub fn run(input: String) -> (i32, i32) {
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for history in input {
        let mut differences: Vec<Vec<i32>> = vec![history.clone()];
        while differences
            .last()
            .unwrap()
            .iter()
            .filter(|x| **x != 0)
            .count()
            != 0
        {
            let latest = &differences[differences.len() - 1];
            let mut changes = vec![];

            for i in 0..(latest.len() - 1) {
                changes.push(latest[i + 1] - latest[i]);
            }

            differences.push(changes);
        }

        let mut num = 0;
        differences.reverse();
        for diff in differences {
            num += diff[diff.len() - 1];
        }

        sum += num;
    }

    sum
}

fn part_two(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for history in input {
        let mut differences: Vec<Vec<i32>> = vec![history.clone()];
        while differences
            .last()
            .unwrap()
            .iter()
            .filter(|x| **x != 0)
            .count()
            != 0
        {
            let latest = &differences[differences.len() - 1];
            let mut changes = vec![];

            for i in 0..(latest.len() - 1) {
                changes.push(latest[i + 1] - latest[i]);
            }

            differences.push(changes);
        }

        let mut num = 0;
        differences.reverse();
        for diff in differences {
            num = diff[0] - num;
        }

        sum += num;
    }

    sum
}
