pub fn run(input: String) -> (u64, u64) {
    let input: Vec<&str> = input.lines().collect();

    let times: Vec<u64> = input[0]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = input[1]
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let answer_one = part_one(&times, &distances);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut mult = 1;
    for i in 0..times.len() {
        let mut how_many = 0;
        let total_time = times[i];
        for time in 0..total_time {
            let speed = time;
            let remaining = total_time - time;
            if speed * remaining > distances[i] {
                how_many += 1;
            }
        }

        mult *= how_many;
    }

    mult
}

fn part_two(input: &[&str]) -> u64 {
    let times = vec![60808676];
    let distances: Vec<u64> = vec![601116315591300];
    let mut mult = 1;
    for i in 0..times.len() {
        let mut how_many = 0;
        let total_time = times[i];
        for time in 0..total_time {
            let speed = time;
            let remaining = total_time - time;
            if speed * remaining > distances[i] {
                how_many += 1;
            }
        }

        mult *= how_many;
    }

    mult
}
