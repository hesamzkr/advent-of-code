#[macro_export]
macro_rules! solve {
    ($year:ident, $md:ident) => {{
        use std::fs;
        let year = $year;
        let day = &stringify!($md)[3..];
        let day_title = format!("--------- Day {day} ---------");
        println!("\n   Advent Of Code {year}");
        println!("{day_title}");
        match fs::read_to_string(format!("inputs/year{year}/input_{day}.txt")) {
            Ok(input_str) => {
                let start_time = std::time::Instant::now();
                let (answer_one, answer_two) = $md::run(input_str);
                let end_time = std::time::Instant::now();
                println!("Part One: {answer_one}");

                println!("Part Two: {answer_two}");

                println!("{}", "-".repeat(day_title.len()));
                println!("Time taken: {:.2?}", end_time - start_time);
            }
            Err(_) => println!("Input file doesn't exist or is unreadable"),
        };
    }};
}

