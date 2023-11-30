#![allow(warnings)]
use std::{
    env, fs,
    io::{self, Write},
};

mod common;
mod solutions;

fn main() {
    let sysargs: Vec<String> = env::args().collect();

    let year;
    let day;

    match sysargs.len() {
        3.. => {
            year = sysargs.get(1).unwrap().to_string();
            day = sysargs.get(2).unwrap().to_string();
        }
        2 => {
            year = "2023".to_string();
            day = sysargs.get(1).unwrap().to_string();
        }
        _ => {
            let mut buffer = String::new();
            print!("\nEnter the year: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();
            year = buffer.trim().to_owned();
            buffer.clear();
            print!("Enter the day number: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();
            day = buffer.trim().to_owned();
        }
    }

    match year.parse() {
        Ok(year) => match day.parse() {
            Ok(day) => solutions::run(year, day),
            Err(_) => {
                if day == "all" {
                    let latest_solution = fs::read_dir(format!("src/solutions/year{year}"))
                        .unwrap()
                        .count() as u16;
                    for day in 1..latest_solution {
                        solutions::run(year, day);
                    }
                } else {
                    println!("Please enter a day number or 'all'");
                }
            }
        },
        Err(_) => {
            println!("Please enter a number for year");
        }
    }
}
