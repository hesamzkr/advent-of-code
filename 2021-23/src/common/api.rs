use std::fs;
use std::fs::File;
use std::io::Write;

pub fn get_input(year: u16, day: &str) -> String {
    let file_path = format!("inputs/year{year}/input_{day}.txt");
    match fs::read_to_string(&file_path) {
        Ok(input_str) => input_str,
        Err(_) => {
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");

            let client = reqwest::blocking::Client::new();

            let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set in env");

            let response = client
                .get(url)
                .header(reqwest::header::COOKIE, format!("session={token}"))
                .send()
                .unwrap();

            if response.status().is_success() {
                let body = response.bytes().unwrap();
                let mut file = File::create(file_path).unwrap();
                file.write_all(&body).unwrap();

                println!("Request successful");

                String::from_utf8_lossy(&body).to_string()
            } else {
                panic!("Request failed with status code: {}", response.status());
            }
        }
    }
}
