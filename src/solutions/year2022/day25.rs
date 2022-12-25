use std::vec;

pub fn run(input: String) -> (String, String) {
    let input: Vec<Vec<i64>> = input
    .lines()
    .map(|line| {
        Vec::from_iter(
            line.chars().rev().map(|c| match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Invalid char"),
            })
         )
    }).collect();

    let answer_one = part_one(&input);

    (answer_one, "N/A".to_string())
}

fn part_one(numbers: &Vec<Vec<i64>>) -> String {
    let mut sum = numbers.iter().fold(0, |acc, number| acc + {
        number.iter().enumerate().fold(0, |acc, (i, digit)| acc + 5_i64.pow(i as u32) * digit)
    });
        
    let mut snafu: Vec<char> = vec![];
    while (sum != 0) {
        snafu.insert(0, "012=-".chars().nth((sum % 5) as usize).unwrap());
        sum = (sum + 2) / 5;
    }
    
    snafu.iter().collect()
}