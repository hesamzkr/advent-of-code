use std::collections::HashMap;

use crate::common::Solution;

pub struct Day7;

pub struct Dir {
    name: String,
    prev: String,
    dirs: Vec<String>,
    files: Vec<(u128, String)>
} 

impl Solution for Day7 {
    type Parsed = Vec<String>;
    type PartOneOutput = u128;
    type PartTwoOutput = u128;

    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.to_string()).collect()

    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut system: Vec<Dir> = vec![Dir {
            name: "/".to_string(),
            prev: "".to_string(),
            dirs: Vec::new(),
            files: Vec::new(),
        }];
        let mut current_dir: usize = 0;
        for line in data {
            if line.starts_with("$") {
                if line.chars().nth(2) == Some('c') {
                    let arg = line.split_whitespace().nth(2).unwrap();
                    current_dir = match arg {
                        "/" => 0,
                        ".." => {
                            system
                            .iter()
                            .position(|dir| dir.name == 
                                system[current_dir].prev && dir.dirs.contains(&system[current_dir].name)).unwrap()
                        },
                        _ => {

                            system
                            .iter()
                            .position(|dir| 
                                dir.name == arg.to_string() && dir.prev == system[current_dir].name
                            ).unwrap()
                        },
                    };
                }
            } else {
                let split: Vec<&str> = line.split_whitespace().collect();
                if split[0] == "dir" {
                    system[current_dir].dirs.push(split[1].to_string());

                    system.push(Dir {
                        name: split[1].to_string(),
                        prev: system[current_dir].name.to_owned(),
                        dirs: Vec::new(),
                        files: Vec::new(),
                    });
                } else {
                    system[current_dir].files.push((split[0].parse().unwrap(), split[1].to_string()));
                }
            }
        }

        let mut total = 0;
        for dir in &system {
            // println!("name: {}, prev: {}, dirs: {:?}, files: {:?}", dir.name, dir.prev, dir.dirs, dir.files);
            let size = Self::calc_dir_size(&system, dir);
            if size < 100000 {
                total += size;
            }
        }

        total
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        0
    }
}

impl Day7 {
    fn calc_dir_size(system: &Vec<Dir>, dir: &Dir) -> u128 {
        let mut size = 0;

        for (file_size, name) in &dir.files {
            // println!("{}: name:{}, size:{}", dir.name, name, file_size);
            size += file_size;
        }

        for sub_dir in &dir.dirs {
            let i = system.iter().position(|x| x.name == 
                                *sub_dir && x.prev == dir.name).unwrap();
            size += Self::calc_dir_size(system, &system[i]);
        }

        // println!("{}, {size}", dir.name);
        size
    }
}
