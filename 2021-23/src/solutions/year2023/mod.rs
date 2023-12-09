use crate::{generate_year_match, import_modules};

import_modules!(day1, day2, day3, day4, day5, day8, day9);

const YEAR: u16 = 2023;

pub fn run(day: u16) {
    let day_string = format!("day{day}");
    let day_str = day_string.as_str();
    generate_year_match!(day_str, day1, day2, day3, day4, day5, day8, day9);
}
