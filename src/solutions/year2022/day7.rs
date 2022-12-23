use std::collections::HashMap;

pub fn run(input: String) -> (u64, u64) {
    let input: Vec<&str> = input.lines().collect();

    let (answer_one, system) = part_one(&input);
    let answer_two = part_two(&system);

    (answer_one, answer_two)
}

fn part_one(input: &[&str]) -> (u64, HashMap<String, u64>) {
    let mut system: HashMap<String, u64> = HashMap::from([("".to_string(), 0)]);
    let mut path: Vec<&str> = Vec::new();
    for line in input {
        if line.starts_with('$') {
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
                let file_size = split[0].parse::<u64>().unwrap();
                let mut temp = path.clone();
                while !temp.is_empty() {
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

    (total, system)
}

fn part_two(system: &HashMap<String, u64>) -> u64 {
    let total: u64 = 70000000;
    let needed: u64 = 30000000;
    let used_space = system.get("").unwrap();
    *system
        .values()
        .filter(|size| total - used_space + **size > needed)
        .min()
        .unwrap()
}
