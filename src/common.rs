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
                let input = $st::parse(input_str);

                println!("Part One: {:?}", $st::part_one(input.clone()));

                println!("Part Two: {:?}", $st::part_two(input.clone()));

                println!("{}", "-".repeat(day_title.len()));
            }
            Err(_) => println!("inputs/input_{day}.txt doesn't exist or is unreadable"),
        };
    }};
}

pub trait Solution {
    type Parsed: Clone;
    type PartOneOutput: Debug;
    type PartTwoOutput: Debug;

    fn parse(input: String) -> Self::Parsed;
    fn part_one(data: Self::Parsed) -> Self::PartOneOutput;
    fn part_two(data: Self::Parsed) -> Self::PartTwoOutput;
}
