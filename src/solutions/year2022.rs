mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
// mod day19;

mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::solve;

const YEAR: u16 = 2022;

pub fn run(day: u16) {
    match day {
        1 => solve!(YEAR, day1),
        2 => solve!(YEAR, day2),
        3 => solve!(YEAR, day3),
        4 => solve!(YEAR, day4),
        5 => solve!(YEAR, day5),
        6 => solve!(YEAR, day6),
        7 => solve!(YEAR, day7),
        8 => solve!(YEAR, day8),
        9 => solve!(YEAR, day9),
        10 => solve!(YEAR, day10),
        11 => solve!(YEAR, day11),
        12 => solve!(YEAR, day12),
        13 => solve!(YEAR, day13),
        14 => solve!(YEAR, day14),
        15 => solve!(YEAR, day15),
        16 => solve!(YEAR, day16),
        // 17 => solve!(YEAR, day17),
        18 => solve!(YEAR, day18),
        // 19 => solve!(YEAR, day19),
        20 => solve!(YEAR, day20),
        21 => solve!(YEAR, day21),
        22 => solve!(YEAR, day22),
        23 => solve!(YEAR, day23),
        24 => solve!(YEAR, day24),
        25 => solve!(YEAR, day25),
        1..=25 => println!("Day {day} is not implemented for {YEAR}"),
        _ => println!("Invalid day number"),
    };
}
