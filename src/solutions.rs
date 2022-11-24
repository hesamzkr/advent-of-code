mod day1;

use crate::solve;
use day1::Day1;

pub fn run(day: u16) {
    match day {
        1 => solve!(struct Day1, day),
        _ => println!("Invalid day number"),
    };
}
