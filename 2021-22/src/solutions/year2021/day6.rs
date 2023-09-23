use std::collections::HashMap;

pub fn run(input: String) -> (usize, i128) {
    let input: Vec<i128> = input.split(",").map(|s| s.parse().unwrap()).collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<i128>) -> usize {
    let mut input = input.to_vec();

    for _ in 0..80 {
        for i in 0..input.len() {
            input[i] -= 1;
            if input[i] < 0 {
                input.push(8);
                input[i] = 6;
            }
        }
    }

    return input.len();
}

fn part_two(input: &Vec<i128>) -> i128 {
    let mut tracker: HashMap<i128, i128> = HashMap::new();

    for i in 0..9 {
        tracker.insert(i, 0);
    }

    for i in input {
        *tracker.get_mut(&i).unwrap() += 1;
    }

    for _ in 0..256 {
        let mut temp1;
        let mut temp2;

        temp1 = *tracker.get(&7).unwrap();
        *tracker.get_mut(&7).unwrap() = *tracker.get(&8).unwrap();

        temp2 = *tracker.get(&6).unwrap();
        *tracker.get_mut(&6).unwrap() = temp1;

        temp1 = *tracker.get(&5).unwrap();
        *tracker.get_mut(&5).unwrap() = temp2;

        temp2 = *tracker.get(&4).unwrap();
        *tracker.get_mut(&4).unwrap() = temp1;

        temp1 = *tracker.get(&3).unwrap();
        *tracker.get_mut(&3).unwrap() = temp2;

        temp2 = *tracker.get(&2).unwrap();
        *tracker.get_mut(&2).unwrap() = temp1;

        temp1 = *tracker.get(&1).unwrap();
        *tracker.get_mut(&1).unwrap() = temp2;

        temp2 = *tracker.get(&0).unwrap();
        *tracker.get_mut(&0).unwrap() = temp1;

        *tracker.get_mut(&6).unwrap() += temp2;
        *tracker.get_mut(&8).unwrap() = temp2;
    }

    let mut count = 0;
    for (_, v) in tracker.iter() {
        count += v;
    }

    return count;
}
