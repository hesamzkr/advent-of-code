pub fn run(input: String) -> (usize, u32) {
    let input = input
        .lines()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split(';')
                .map(|picks| {
                    picks
                        .split(',')
                        .map(|pick| {
                            let x = pick.split_whitespace().collect::<Vec<&str>>();
                            (x[0].parse::<u32>().unwrap(), x[1])
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(games: &Vec<Vec<Vec<(u32, &str)>>>) -> usize {
    let mut sum = 0;
    for (i, game) in games.iter().enumerate() {
        let mut game_is_valid = true;
        for round in game {
            for (num, color) in round {
                let check = match *color {
                    "red" => *num > 12,
                    "green" => *num > 13,
                    "blue" => *num > 14,
                    _ => panic!("joerover"),
                };
                if check {
                    game_is_valid = false;
                }
            }
        }

        if game_is_valid {
            sum += i + 1;
        }
    }

    sum
}

fn part_two(games: &Vec<Vec<Vec<(u32, &str)>>>) -> u32 {
    let mut sum = 0;
    for (i, game) in games.iter().enumerate() {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for round in game {
            for (num, color) in round {
                let check = match *color {
                    "red" => {
                        if num > &max_red {
                            max_red = *num;
                        }
                    }
                    "green" => {
                        if num > &max_green {
                            max_green = *num;
                        }
                    }
                    "blue" => {
                        if num > &max_blue {
                            max_blue = *num;
                        }
                    }
                    _ => panic!("joerover"),
                };
            }
        }

        sum += max_red * max_green * max_blue;
    }

    sum
}
