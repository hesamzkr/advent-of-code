#[macro_export]
macro_rules! solve {
    ($year:ident, $md:ident) => {{
        use crate::common::api::get_input;
        use std::fs;
        let year = $year;
        let day = &stringify!($md)[3..];
        let day_title = format!("--------- Day {day} ---------");
        let year_string = format!("Advent Of Code {year}");
        let spaces = " ".repeat((day_title.len() - year_string.len()) / 2);
        println!("\n{}{}{}", spaces, year_string, spaces);
        println!("{day_title}");

        let input = get_input(year, day);

        let start_time = std::time::Instant::now();
        let (answer_one, answer_two) = $md::run(input);
        let end_time = std::time::Instant::now();
        println!("Part One: {answer_one}");

        println!("Part Two: {answer_two}");

        println!("{}", "-".repeat(day_title.len()));
        println!("Time taken: {:.2?}", end_time - start_time);
    }};
}

#[macro_export]
macro_rules! generate_year_match {
    ($value:ident, $($module:ident),* ) => {
        match $value {
            $(
                stringify!($module) => crate::solve!(YEAR, $module),
            )*
            _ => println!("Invalid day number"),
        };
    };
}

#[macro_export]
macro_rules! import_modules {
    ( $( $module:ident ),* ) => {
        $(
            mod $module;
        )*
    };
}
