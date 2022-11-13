#![allow(warnings)]
use std::env;
use std::io;

mod solutions;

fn main() {
    let sysargs: Vec<String> = env::args().collect();

    match match sysargs.get(1) {
        Some(x) => x.to_string(),
        None => {
            let mut day_str = String::new();
            println!("Enter the question number:");
            io::stdin().read_line(&mut day_str);
            println!("");
            day_str
        }
    }
    .trim()
    .parse()
    {
        Ok(day_num) => solutions::run(day_num),
        Err(e) => println!("argument is not a number"),
    };
}
