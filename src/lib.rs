use std::fmt::Debug;

#[macro_export]
macro_rules! solve {
    ($st:ident, $day:ident) => {{
        use crate::lib::Solution;
        use std::fs;
        println!("\nDay {}", $day);
        let day = $day.to_string();
        let input_str =
            fs::read_to_string(format!("inputs/input_{day}.txt")).expect("Failed to read file");
        let input = $st::parse(input_str);

        println!("Part One: {}", $st::part_one(input.clone()));

        println!("Part Two: {}", $st::part_two(input.clone()));
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
