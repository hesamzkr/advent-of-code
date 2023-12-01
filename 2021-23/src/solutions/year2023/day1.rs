pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &[&str]) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut numbers = vec![];
        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char);
            }
        }

        sum += format!("{}{}", numbers[0], numbers[numbers.len() - 1])
            .parse::<u32>()
            .unwrap();
    }

    sum
}

fn part_two(input: &[&str]) -> u32 {
    let mut sum = 0;
    let word_numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut numbers: Vec<(String, usize)> = vec![];

        for (i, char) in line.chars().enumerate() {
            if char.is_numeric() {
                numbers.push((char.to_string(), i));
            }
        }

        for (i, word) in word_numbers.iter().enumerate() {
            if let indices = line.match_indices(word).collect::<Vec<_>>() {
                for index in indices {
                    numbers.push((format!("{}", i + 1), index.0));
                }
            }
        }

        numbers.sort_by(|a, b| a.1.cmp(&b.1));

        sum += format!("{}{}", numbers[0].0, numbers[numbers.len() - 1].0)
            .parse::<u32>()
            .unwrap();
    }

    sum
}
