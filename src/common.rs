use std::fmt::Debug;

#[macro_export]
macro_rules! solve {
    ($st:ident, $day:ident) => {{
        use crate::common::Solution;
        use std::fs;
        let day = $day.to_string();
        let day_title = format!("--- Day {day} ---");
        println!("\n{}", day_title);
        match fs::read_to_string(format!("inputs/input_{day}.txt")) {
            Ok(input_str) => {
                let mut input = $st::parse(input_str);

                println!("Part One: {:?}", $st::part_one(&mut input));

                println!("Part Two: {:?}", $st::part_two(&mut input));

                println!("{}", "-".repeat(day_title.len()));
            }
            Err(_) => println!("inputs/input_{day}.txt doesn't exist or is unreadable"),
        };
    }};
}

pub trait Solution {
    type Parsed;
    type PartOneOutput: Debug;
    type PartTwoOutput: Debug;

    fn parse(input: String) -> Self::Parsed;
    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput;
    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput;
}
