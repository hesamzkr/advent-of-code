#[macro_export]
macro_rules! solve {
    ($md:ident, $day:ident) => {{
        use std::fs;
        let day = $day.to_string();
        let day_title = format!("--- Day {day} ---");
        println!("\n{}", day_title);
        match fs::read_to_string(format!("inputs/input_{day}.txt")) {
            Ok(input_str) => {
                let time = std::time::Instant::now();
                let answers = $md::run(input_str);

                println!("Part One: {:?}", answers.0);

                println!("Part Two: {:?}", answers.1);

                println!("{}", "-".repeat(day_title.len()));
                println!("Time taken: {:.2?}", std::time::Instant::now() - time);
            }
            Err(_) => println!("inputs/input_{day}.txt doesn't exist or is unreadable"),
        };
    }};
}
