use std::fs;

pub fn run() {
    let input_str = utils::read_input(1);

    // dividers: "," | "\n" |
    let input = utils::list_of_strings(&input_str, "");
    let input = utils::list_of_integers(&input_str, "");
    let input = utils::list_of_list_strings(&input_str, "");
    let input = utils::list_of_list_integers(&input_str, "");

    println!("Part One: {}", part_one());
    // println!("Part Two: {}", part_two());
}

fn part_one() {}

fn part_two() {}
