pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    (part_one, part_two)
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let tree = input[i][j];
            let mut is_visible = true;
            for x_pos in j + 1..input[0].len() {
                if tree <= input[i][x_pos] {
                    is_visible = false;
                }
            }
            if is_visible {
                count += 1;
                continue;
            }
            is_visible = true;
            for x_neg in 0..j {
                if tree <= input[i][x_neg] {
                    is_visible = false;
                }
            }
            if is_visible {
                count += 1;
                continue;
            }
            is_visible = true;
            for y_pos in 0..i {
                if tree <= input[y_pos][j] {
                    is_visible = false;
                }
            }
            if is_visible {
                count += 1;
                continue;
            }
            is_visible = true;
            for y_neg in i + 1..input.len() {
                if tree <= input[y_neg][j] {
                    is_visible = false;
                }
            }
            if is_visible {
                count += 1;
                continue;
            }
        }
    }

    count
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    0
}
