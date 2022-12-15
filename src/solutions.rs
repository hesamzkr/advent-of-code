mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
        1 => solve!(day1),
        2 => solve!(day2),
        3 => solve!(day3),
        4 => solve!(day4),
        5 => solve!(day5),
        6 => solve!(day6),
        7 => solve!(day7),
        8 => solve!(day8),
        9 => solve!(day9),
        10 => solve!(day10),
        11 => solve!(day11),
        12 => solve!(day12),
        13 => solve!(day13),
        14 => solve!(day14),
        15 => solve!(day15),
        // 16 => solve!(day16),
        // 17 => solve!(day17),
        // 18 => solve!(day18),
        // 19 => solve!(day19),
        // 20 => solve!(day20),
        // 21 => solve!(day21),
        // 22 => solve!(day22),
        // 23 => solve!(day23),
        // 24 => solve!(day24),
        // 25 => solve!(day25),
        _ => println!("Invalid day number"),
    };
}
