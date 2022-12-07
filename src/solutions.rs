mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
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

pub fn run(day: u16) {
    match day {
        1 => solve!(day1, day),
        2 => solve!(day2, day),
        3 => solve!(day3, day),
        4 => solve!(day4, day),
        5 => solve!(day5, day),
        6 => solve!(day6, day),
        7 => solve!(day7, day),
        8 => solve!(day8, day),
        // 9 => solve!(day9, day),
        // 10 => solve!(day10, day),
        // 11 => solve!(day11, day),
        // 12 => solve!(day12, day),
        // 13 => solve!(day13, day),
        // 14 => solve!(day14, day),
        // 15 => solve!(day15, day),
        // 16 => solve!(day16, day),
        // 17 => solve!(day17, day),
        // 18 => solve!(day18, day),
        // 19 => solve!(day19, day),
        // 20 => solve!(day20, day),
        // 21 => solve!(day21, day),
        // 22 => solve!(day22, day),
        // 23 => solve!(day23, day),
        // 24 => solve!(day24, day),
        // 25 => solve!(day25, day),
        _ => println!("Invalid day number"),
    };
}
