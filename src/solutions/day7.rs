use std::collections::HashMap;

use crate::common::Solution;

pub struct Day7;

impl Solution for Day7 {
    type Parsed = Vec<String>;
    type PartOneOutput = u128;
    type PartTwoOutput = u128;
    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.to_string()).collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut system: HashMap<String, u128> = HashMap::from([("".to_string(), 0)]);
        let mut path: Vec<&str> = Vec::new();
        for line in data {
            if line.starts_with("$") {
                if line.chars().nth(2) == Some('c') {
                    let dir = line.split_whitespace().nth(2).unwrap();
                    if dir == "/" {
                        path = Vec::new();
                    } else if dir == ".." {
                        path.pop();
                    } else {
                        path.push(dir);
                    }
                }
            } else {
                let split: Vec<&str> = line.split_whitespace().collect();
                if split[0] == "dir" {
                    path.push(split[1]);
                    system.insert(path.join("/"), 0);
                    path.pop();
                } else {
                    let file_size = split[0].parse::<u128>().unwrap();
                    let mut temp = path.clone();
                    while temp.len() > 0 {
                        *system.get_mut(&temp.join("/")).unwrap() += file_size;
                        temp.pop();
                    }
                    *system.get_mut(&temp.join("/")).unwrap() += file_size;
                    temp.pop();
                }
            }
        }

        let mut total = 0;
        for size in system.values() {
            if size < &100000 {
                total += size;
            }
        }

        total
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut system: HashMap<String, u128> = HashMap::from([("".to_string(), 0)]);
        let mut path: Vec<&str> = Vec::new();
        for line in data {
            if line.starts_with("$") {
                if line.chars().nth(2) == Some('c') {
                    let dir = line.split_whitespace().nth(2).unwrap();
                    if dir == "/" {
                        path = Vec::new();
                    } else if dir == ".." {
                        path.pop();
                    } else {
                        path.push(dir);
                    }
                }
            } else {
                let split: Vec<&str> = line.split_whitespace().collect();
                if split[0] == "dir" {
                    path.push(split[1]);
                    system.insert(path.join("/"), 0);
                    path.pop();
                } else {
                    let file_size = split[0].parse::<u128>().unwrap();
                    let mut temp = path.clone();
                    while temp.len() > 0 {
                        *system.get_mut(&temp.join("/")).unwrap() += file_size;
                        temp.pop();
                    }
                    *system.get_mut(&temp.join("/")).unwrap() += file_size;
                    temp.pop();
                }
            }
        }

        let total: u128 = 70000000;
        let needed: u128 = 30000000;
        let used_space = system.get("").unwrap();
        *system
            .values()
            .filter(|size| total - used_space + **size > needed)
            .min()
            .unwrap()
    }
}
