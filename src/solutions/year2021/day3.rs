pub fn run(input: String) -> (isize, isize) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &[&str]) -> isize {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..input[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;
        for j in 0..input.len() {
            if input[j].to_string().chars().nth(i).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        if zero_count > one_count {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

fn part_two(input: &[&str]) -> isize {
    let mut oxygen_rate = 0;
    let mut co2_rate = 0;

    let mut removed_indices: Vec<usize> = vec![];

    for i in 0..input[0].len() {
        if input.len() == removed_indices.len() + 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for j in 0..input.len() {
            if !removed_indices.contains(&j) {
                if input[j].to_string().chars().nth(i).unwrap() == '0' {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }
            }
        }

        if zero_count > one_count {
            for x in 0..input.len() {
                if !removed_indices.contains(&x) && input[x].to_string().chars().nth(i).unwrap() == '1' {
                    removed_indices.push(x);
                }
            }
        } else {
            for x in 0..input.len() {
                if !removed_indices.contains(&x) && input[x].to_string().chars().nth(i).unwrap() == '0' {
                    removed_indices.push(x);
                }
            }
        }
    }

    
    for d in 0..input.len() {
        if !removed_indices.contains(&d) {
            oxygen_rate = isize::from_str_radix(&input[d], 2).unwrap();
        }
    }

    removed_indices = vec![];
    for i in 0..input[0].len() {
        if input.len() == removed_indices.len() + 1 {
            break;
        }
        let mut zero_count = 0;
        let mut one_count = 0;
        for j in 0..input.len() {
            if !removed_indices.contains(&j) {
                if input[j].to_string().chars().nth(i).unwrap() == '0' {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }
            }
        }

        if zero_count > one_count {
            for x in 0..input.len() {
                if !removed_indices.contains(&x) && input[x].to_string().chars().nth(i).unwrap() == '0' {
                    removed_indices.push(x);
                }
            }
        } else {
            for x in 0..input.len() {
                if !removed_indices.contains(&x) && input[x].to_string().chars().nth(i).unwrap() == '1' {
                    removed_indices.push(x);
                }
            }
        }
    }

    
    for d in 0..input.len() {
        if !removed_indices.contains(&d) {
            co2_rate = isize::from_str_radix(&input[d], 2).unwrap();
        }
    }

    oxygen_rate * co2_rate
}
