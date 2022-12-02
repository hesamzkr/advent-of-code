mod day1;
mod day2;
mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;

use crate::solve;
use day1::Day1;
use day2::Day2;
use day3::Day3;
// use day4::Day4;
// use day5::Day5;
// use day6::Day6;
// use day7::Day7;
// use day8::Day8;
// use day9::Day9;
// use day10::Day10;

pub fn run(day: u16) {
    match day {
        1 => solve!(Day1, day),
        2 => solve!(Day2, day),
        3 => solve!(Day3, day),
        // 4 => solve!(Day4, day),
        // 5 => solve!(Day5, day),
        // 6 => solve!(Day6, day),
        // 7 => solve!(Day7, day),
        // 8 => solve!(Day8, day),
        // 9 => solve!(Day9, day),
        // 10 => solve!(Day10, day),
        _ => println!("Invalid day number"),
    };
}
