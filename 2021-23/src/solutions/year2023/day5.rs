use std::convert;

#[derive(Debug)]
struct Convert {
    destination: u32,
    source: u32,
    length: u32,
}

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let seeds = input[0].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut parsed: Vec<Vec<Convert>> = vec![];
    for i in 1..input.len() {
        let converts: Vec<Convert> = input[i]
            .lines()
            .skip(1)
            .map(|line| {
                let numbers: Vec<u32> = line
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();

                Convert {
                    destination: numbers[0],
                    source: numbers[1],
                    length: numbers[2],
                }
            })
            .collect();

        parsed.push(converts);
    }

    let answer_one = part_one(&seeds, &parsed);
    let answer_two = part_two(&parsed);

    (answer_one, answer_two)
}

fn part_one(seeds: &Vec<u32>, maps: &Vec<Vec<Convert>>) -> u32 {
    for seed in seeds {
        let mut last = *seed;
        for map in maps {
            for convert in map {
                if last >= convert.source
                    && last <= convert.source + convert.length + convert.length - 1
                {
                }
            }
        }
    }

    0
}

fn part_two(input: &Vec<Vec<Convert>>) -> u32 {
    0
}
