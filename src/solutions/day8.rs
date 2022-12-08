pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    let mut visible_trees = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let tree = input[row][col];
            let col_cells: Vec<u32> = input.iter().map(|x| x[col]).collect();

            let trees_left = input[row].iter().take(col);
            let trees_right = input[row].iter().skip(col + 1);
            let trees_up = col_cells.iter().take(row);
            let trees_down = col_cells.iter().skip(row + 1);

            if trees_left.filter(|x| x >= &&tree).count() == 0
                || trees_right.filter(|x| x >= &&tree).count() == 0
                || trees_up.filter(|x| x >= &&tree).count() == 0
                || trees_down.filter(|x| x >= &&tree).count() == 0
            {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    let mut highest = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let tree = input[row][col];
            let mut scenic_score = 1;
            let col_cells = input.iter().map(|x| x[col]);

            let trees_left = input[row].iter().take(col).rev();
            let trees_right = input[row].iter().skip(col + 1);
            let trees_up = col_cells.clone().take(row).rev();
            let trees_down = col_cells.clone().skip(row + 1);

            let mut count = 0;
            for x in trees_left {
                count += 1;
                if &tree <= x {
                    break;
                }
            }
            scenic_score *= count;

            count = 0;
            for x in trees_right {
                count += 1;
                if &tree <= x {
                    break;
                }
            }
            scenic_score *= count;

            count = 0;
            for x in trees_up {
                count += 1;
                if tree <= x {
                    break;
                }
            }
            scenic_score *= count;

            count = 0;
            for x in trees_down {
                count += 1;
                if tree <= x {
                    break;
                }
            }
            scenic_score *= count;

            if scenic_score > highest {
                highest = scenic_score;
            }
        }
    }

    highest
}
