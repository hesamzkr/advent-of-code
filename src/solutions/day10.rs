use std::vec;

pub fn run(input: String) -> (i32, String) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<&str>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for instruction in input {
        cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += cycle * x;
        }

        if instruction.starts_with("addx") {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum += cycle * x;
            }
            x += instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }
    sum
}

fn part_two(input: &Vec<&str>) -> String {
    let mut grid: Vec<String> = vec![String::new()];
    let mut grid_index = 0;
    let mut cycle = 0;
    let mut x = 1;

    for instruction in input {
        cycle += 1;
        if grid[grid_index].len() == 40 {
            grid.push(String::new());
            grid_index += 1;
            cycle = 1;
        }

        if x + 1 == cycle || x == cycle || x + 2 == cycle {
            grid[grid_index].push('#');
        } else {
            grid[grid_index].push('.');
        }

        if instruction.starts_with("addx") {
            cycle += 1;
            if grid[grid_index].len() == 40 {
                grid.push(String::new());
                grid_index += 1;
                cycle = 1;
            }

            if x + 1 == cycle || x == cycle || x + 2 == cycle {
                grid[grid_index].push('#');
            } else {
                grid[grid_index].push('.');
            }

            x += instruction
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }

    "\n".to_owned() + &grid.join("\n")
}
