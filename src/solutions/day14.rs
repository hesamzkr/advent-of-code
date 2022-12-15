use std::collections::HashSet;

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<(u32, u32)>> = input
        .lines()
        .map(|x| {
            x.split(" -> ")
                .map(|y| {
                    let mut y = y.split(",");
                    (
                        y.next().unwrap().parse().unwrap(),
                        y.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let (grid, lowest_y) = make_grid(&input);

    let answer_one = part_one(grid.clone(), lowest_y);
    let answer_two = part_two(grid, lowest_y + 2);

    (answer_one, answer_two)
}

fn part_one(mut grid: HashSet<(u32, u32)>, lowest_y: u32) -> u32 {
    let mut count = 0;
    'outer: loop {
        let mut position = (500, 0);
        'inner: loop {
            if position.1 > lowest_y {
                break 'outer;
            }

            if grid.contains(&(position.0, position.1 + 1)) {
                if grid.contains(&(position.0 - 1, position.1 + 1)) {
                    if grid.contains(&(position.0 + 1, position.1 + 1)) {
                        grid.insert(position);
                        count += 1;
                        break 'inner;
                    } else {
                        position = (position.0 + 1, position.1 + 1);
                    }
                } else {
                    position = (position.0 - 1, position.1 + 1);
                }
            } else {
                position = (position.0, position.1 + 1);
            }
        }
    }

    count
}

fn part_two(mut grid: HashSet<(u32, u32)>, lowest_y: u32) -> u32 {
    let mut count = 0;
    'outer: loop {
        let mut position = (500, 0);
        'inner: loop {
            if grid.contains(&(500, 0)) {
                break 'outer;
            }

            if position.1 + 1 == lowest_y {
                grid.insert(position);
                count += 1;
                continue 'outer;
            }

            if grid.contains(&(position.0, position.1 + 1)) {
                if grid.contains(&(position.0 - 1, position.1 + 1)) {
                    if grid.contains(&(position.0 + 1, position.1 + 1)) {
                        grid.insert(position);
                        count += 1;
                        break 'inner;
                    } else {
                        position = (position.0 + 1, position.1 + 1);
                    }
                } else {
                    position = (position.0 - 1, position.1 + 1);
                }
            } else {
                position = (position.0, position.1 + 1);
            }
        }
    }
    count
}

fn make_grid(paths: &Vec<Vec<(u32, u32)>>) -> (HashSet<(u32, u32)>, u32) {
    let mut grid: HashSet<(u32, u32)> = HashSet::new();
    for path in paths {
        for i in 0..path.len() - 1 {
            let start = path[i];
            let finish = path[i + 1];
            let x_range = if finish.0 > start.0 {
                (start.0..finish.0 + 1)
            } else {
                (finish.0..start.0)
            };

            let y_range = if finish.1 > start.1 {
                (start.1..finish.1 + 1)
            } else {
                (finish.1..start.1)
            };

            for x in x_range {
                grid.insert((x, start.1));
            }
            for y in y_range {
                grid.insert((start.0, y));
            }
        }
    }

    let lowest_y = grid.iter().map(|tuple| tuple.1).max().unwrap();

    (grid, lowest_y)
}
