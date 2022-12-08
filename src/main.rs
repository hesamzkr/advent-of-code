#![allow(warnings)]
use std::{env, fs, io};

mod common;
mod solutions;

fn main() {
    let sysargs: Vec<String> = env::args().collect();

    let day_str = match sysargs.get(1) {
        Some(x) => x.to_string(),
        None => {
            let mut buffer = String::new();
            println!("Enter the question number:");
            io::stdin().read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    };

    match day_str.parse() {
        Ok(day_num) => solutions::run(day_num),
        Err(_) => {
            if day_str == "all" {
                let dir = fs::read_dir("./src/solutions").unwrap();
                let latest_solution = dir.fold(0, |i, _| i + 1);
                for day_num in 1..latest_solution {
                    solutions::run(day_num);
                }
            } else {
                println!("Please enter a number or 'all'");
            }
        }
    };
}
