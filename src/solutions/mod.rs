mod year2022;

pub fn run(year: u16, day: u16) {
    match year {
        2022 => year2022::run(day),

        2015..=2022 => println!("Year {year} is not implemented"),
        _ => println!("Invalid year"),
    }
}