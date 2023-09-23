pub fn run(input: String) -> (i32, i32) {
    let input: Vec<&str> = input.lines().collect();

    let rand_nums: Vec<i32> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<Vec<Vec<i32>>> = vec![];
    let mut marked_nums: Vec<Vec<Vec<i32>>> = vec![];

    for i in 1..input.len() {
        if input[i] == "" {
            boards.push(vec![]);
            marked_nums.push(vec![]);
        } else {
            let x: Vec<i32> = input[i]
                .split(" ")
                .filter(|&s| s != "")
                .map(|s| s.parse().unwrap())
                .collect();
            let board_length = boards.len();
            boards[board_length - 1].push(x);
            marked_nums[board_length - 1].push(vec![0, 0, 0, 0, 0]);
        }
    }

    let answer_one = part_one(&boards, &marked_nums, &rand_nums);
    let answer_two = part_two(&boards, &marked_nums, &rand_nums);

    (answer_one, answer_two)
}

fn part_one(
    boards: &Vec<Vec<Vec<i32>>>,
    marked_nums: &Vec<Vec<Vec<i32>>>,
    rand_nums: &Vec<i32>,
) -> i32 {
    let boards = boards.to_vec();
    let mut marked_nums = marked_nums.to_vec();
    let rand_nums = rand_nums.to_vec();

    for rand in rand_nums {
        for b in 0..boards.len() {
            for row in 0..5 {
                for col in 0..5 {
                    if rand == boards[b][row][col] {
                        marked_nums[b][row][col] = 1;
                    }
                }
            }
        }

        for b in 0..boards.len() {
            for row in 0..5 {
                let mut row_count = 0;
                let mut col_count = 0;
                for col in 0..5 {
                    if marked_nums[b][row][col] == 1 {
                        row_count += 1;
                    }

                    if marked_nums[b][col][row] == 1 {
                        col_count += 1;
                    }
                }

                if row_count == 5 || col_count == 5 {
                    let mut sum = 0;
                    for i in 0..5 {
                        for j in 0..5 {
                            if marked_nums[b][i][j] == 0 {
                                sum += boards[b][i][j];
                            }
                        }
                    }
                    return sum * rand;
                }
            }
        }
    }

    return 0;
}

fn part_two(
    boards: &Vec<Vec<Vec<i32>>>,
    marked_nums: &Vec<Vec<Vec<i32>>>,
    rand_nums: &Vec<i32>,
) -> i32 {
    let boards = boards.to_vec();
    let mut marked_nums = marked_nums.to_vec();
    let rand_nums = rand_nums.to_vec();

    let mut boards_won: Vec<usize> = vec![];
    for rand in rand_nums {
        for b in 0..boards.len() {
            for row in 0..5 {
                for col in 0..5 {
                    if rand == boards[b][row][col] {
                        marked_nums[b][row][col] = 1;
                    }
                }
            }
        }

        for b in 0..boards.len() {
            if boards_won.contains(&b) {
                continue;
            }

            for row in 0..5 {
                let mut row_count = 0;
                let mut col_count = 0;
                for col in 0..5 {
                    if marked_nums[b][row][col] == 1 {
                        row_count += 1;
                    }

                    if marked_nums[b][col][row] == 1 {
                        col_count += 1;
                    }
                }

                if row_count == 5 || col_count == 5 {
                    boards_won.push(b);
                    break;
                }
            }
        }

        if boards_won.len() == boards.len() {
            let mut sum = 0;
            let last_board = boards_won[boards_won.len() - 1];
            for i in 0..5 {
                for j in 0..5 {
                    if marked_nums[last_board][i][j] == 0 {
                        sum += boards[last_board][i][j];
                    }
                }
            }

            return sum * rand;
        }
    }

    return 0;
}
