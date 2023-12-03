use regex::{Match, Regex};

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    let pattern = Regex::new(r"\d+").unwrap();

    for (i, line) in input.iter().enumerate() {
        let line_string = line.into_iter().collect::<String>();
        let matches: Vec<Match> = pattern.find_iter(line_string.as_str()).collect();

        for m in matches {
            let mut is_valid = false;
            let number = m.as_str().parse::<u32>().unwrap();
            let start_index = m.start();
            let end_index = m.end();

            for j in start_index..end_index {
                if i > 0 && j > 0 && is_symbol(input[i - 1][j - 1]) {
                    is_valid = true;
                }
                if i > 0 && is_symbol(input[i - 1][j]) {
                    is_valid = true;
                }
                if i > 0 && j < line.len() - 1 && is_symbol(input[i - 1][j + 1]) {
                    is_valid = true;
                }

                if j < line.len() - 1 && is_symbol(input[i][j + 1]) {
                    is_valid = true;
                }
                if j > 0 && is_symbol(input[i][j - 1]) {
                    is_valid = true;
                }

                if i < input.len() - 1 && j > 0 && is_symbol(input[i + 1][j - 1]) {
                    is_valid = true;
                }
                if i < input.len() - 1 && is_symbol(input[i + 1][j]) {
                    is_valid = true;
                }
                if i < input.len() - 1 && j < line.len() - 1 && is_symbol(input[i + 1][j + 1]) {
                    is_valid = true;
                }
            }

            if is_valid {
                sum += number
            }
        }
    }

    sum
}

fn part_two(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    let pattern = Regex::new(r"\d+").unwrap();

    let mut parts: Vec<(u32, usize, usize)> = vec![];

    for (i, line) in input.iter().enumerate() {
        let line_string = line.into_iter().collect::<String>();
        let matches: Vec<Match> = pattern.find_iter(line_string.as_str()).collect();

        for m in matches {
            let mut star_index: Option<(usize, usize)> = None;
            let number = m.as_str().parse::<u32>().unwrap();
            let start_index = m.start();
            let end_index = m.end();

            for j in start_index..end_index {
                if i > 0 && j > 0 && input[i - 1][j - 1] == '*' {
                    star_index = Some((i - 1, j - 1));
                }
                if i > 0 && input[i - 1][j] == '*' {
                    star_index = Some((i - 1, j));
                }
                if i > 0 && j < line.len() - 1 && input[i - 1][j + 1] == '*' {
                    star_index = Some((i - 1, j + 1));
                }

                if j < line.len() - 1 && input[i][j + 1] == '*' {
                    star_index = Some((i, j + 1));
                }
                if j > 0 && input[i][j - 1] == '*' {
                    star_index = Some((i, j - 1));
                }

                if i < input.len() - 1 && j > 0 && input[i + 1][j - 1] == '*' {
                    star_index = Some((i + 1, j - 1));
                }
                if i < input.len() - 1 && input[i + 1][j] == '*' {
                    star_index = Some((i + 1, j));
                }
                if i < input.len() - 1 && j < line.len() - 1 && input[i + 1][j + 1] == '*' {
                    star_index = Some((i + 1, j + 1));
                }
            }

            if let Some((row, col)) = star_index {
                parts.push((number, row, col));
            }
        }
    }

    for i in 0..parts.len() - 1 {
        for j in (i + 1)..parts.len() {
            if parts[i].1 == parts[j].1 && parts[i].2 == parts[j].2 {
                sum += parts[i].0 * parts[j].0
            }
        }
    }

    sum
}

fn is_symbol(char: char) -> bool {
    char != '.' && !char.is_numeric()
}
