mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

use crate::solve;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
// use day8::Day8;
// use day9::Day9;
// use day10::Day10;
// use day11::Day11;
// use day12::Day12;
// use day13::Day13;
// use day14::Day14;
// use day15::Day15;
// use day16::Day16;
// use day17::Day17;
// use day18::Day18;
// use day19::Day19;
// use day20::Day20;
// use day21::Day21;
// use day22::Day22;
// use day23::Day23;
// use day24::Day24;
// use day25::Day25;

pub fn run(day: u16) {
    match day {
        1 => solve!(Day1, day),
        2 => solve!(Day2, day),
        3 => solve!(Day3, day),
        4 => solve!(Day4, day),
        5 => solve!(Day5, day),
        6 => solve!(Day6, day),
        7 => solve!(Day7, day),
        // 8 => solve!(Day8, day),
        // 9 => solve!(Day9, day),
        // 10 => solve!(Day10, day),
        // 11 => solve!(Day11, day),
        // 12 => solve!(Day12, day),
        // 13 => solve!(Day13, day),
        // 14 => solve!(Day14, day),
        // 15 => solve!(Day15, day),
        // 16 => solve!(Day16, day),
        // 17 => solve!(Day17, day),
        // 18 => solve!(Day18, day),
        // 19 => solve!(Day19, day),
        // 20 => solve!(Day20, day),
        // 21 => solve!(Day21, day),
        // 22 => solve!(Day22, day),
        // 23 => solve!(Day23, day),
        // 24 => solve!(Day24, day),
        // 25 => solve!(Day25, day),
        _ => println!("Invalid day number"),
    };
}
