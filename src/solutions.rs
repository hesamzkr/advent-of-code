mod day_1;

pub fn run(day: u16) {
    println!("\nDay {}", day);
    match day {
        1 => day_1::run(),
        _ => println!("Invalid day number"),
    };
}

pub fn latest_solution() -> u16 {
    1
}
