#![allow(warnings)]
use std::env;
use std::io;

mod solutions;

fn main() {
    let sysargs: Vec<String> = env::args().collect();

    let day_str = match sysargs.get(1) {
        Some(x) => x.to_string(),
        None => {
            let mut day_str = String::new();
            println!("Enter the question number:");
            io::stdin().read_line(&mut day_str);
            day_str.trim().to_string()
        }
    };

    match day_str.parse() {
        Ok(day_num) => solutions::run(day_num),
        Err(_) => {
            if (day_str == "all") {
                for day_num in 1..solutions::latest_solution() + 1 {
                    solutions::run(day_num);
                }
            } else {
                println!("Please enter a number or 'all'");
            }
        }
    };
}
