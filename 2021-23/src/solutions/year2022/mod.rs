use crate::{generate_year_match, import_modules};

import_modules!(
    day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15,
    day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);

const YEAR: u16 = 2022;

pub fn run(day: u16) {
    let day_string = format!("day{day}");
    let day_str = day_string.as_str();
    generate_year_match!(
        day_str, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
        day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
    );
}
