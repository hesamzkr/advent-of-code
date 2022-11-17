use std::fs;

pub fn read_input(day_num: u16) -> String {
    fs::read_to_string(format!("inputs/input_{day_num}.txt")).expect("Failed to read file")
}

pub fn list_of_strings<'a>(input: &'a str, divider: &'a str) -> Vec<&'a str> {
    input.split(divider).collect()
}

pub fn list_of_integers(input: &str, divider: &str) -> Vec<i32> {
    input
        .split(divider)
        .map(|s| s.parse().expect("Parse error"))
        .collect()
}

pub fn list_of_list_strings<'a>(input: &'a str, divider: &'a str) -> Vec<Vec<&'a str>> {
    input
        .split(divider)
        .map(|s| s.split(",").collect())
        .collect()
}

pub fn list_of_list_integers(input: &str, divider: &str) -> Vec<Vec<i32>> {
    input
        .split(divider)
        .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect())
        .collect()
}
