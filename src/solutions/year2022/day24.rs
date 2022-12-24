use std::collections::{HashMap, HashSet};
use std::mem::swap;

type Point = (usize, usize);

enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub fn run(input: String) -> (usize, usize) {
    
    let mut blizzards: HashMap<Point, Vec<Direction>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                blizzards.entry((x, y)).or_default().push(match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => panic!("Invalid character in input"),
                })
            }
        }
        height += 1;
    }

    let mut current: HashSet<Point> = HashSet::new();
    let mut next: HashSet<Point> = HashSet::new();
    
    let answer_one = part_one(&mut blizzards, width, height, &mut current, &mut next);
    let answer_two = answer_one + part_two(&mut blizzards, width, height, &mut current, &mut next);

    (answer_one, answer_two)
}

fn part_one(blizzards: &mut HashMap<Point, Vec<Direction>>, width: usize, height: usize, current: &mut HashSet<Point>, next: &mut HashSet<Point>) -> usize {
    1 + find_path((1, 0),
    (width - 2, height - 2),
    blizzards,
    width,
    height,
    current,
    next)
}

fn part_two(blizzards: &mut HashMap<Point, Vec<Direction>>, width: usize, height: usize, current: &mut HashSet<Point>, next: &mut HashSet<Point>) -> usize {
    current.clear();
    let back_to_start = find_path(
        (width - 2, height - 1),
        (1, 1),
        blizzards,
        width,
        height,
        current,
        next,
    );

    current.clear();
    let back_to_goal = find_path(
        (1, 0),
        (width - 2, height - 2),
        blizzards,
        width,
        height,
        current,
        next,
    );

    back_to_start + back_to_goal
}

fn find_path(
    start: (usize, usize),
    goal: (usize, usize),
    blizzards: &mut HashMap<Point, Vec<Direction>>,
    width: usize,
    height: usize,
    current: &mut HashSet<Point>,
    next: &mut HashSet<Point>,
) -> usize {
    let mut minute = 0;
    current.insert(start);
    loop {
        let mut to_add = Vec::new();
        for pos in blizzards.keys().copied().collect::<Vec<_>>() {
            let (x, y) = pos;
            for dir in blizzards.get_mut(&pos).unwrap().drain(0..) {
                match dir {
                    Direction::Right => {
                        if x >= width - 2 {
                            to_add.push(((1, y), dir));
                        } else {
                            to_add.push(((x + 1, y), dir));
                        }
                    }
                    Direction::Left => {
                        if x <= 1 {
                            to_add.push(((width - 2, y), dir));
                        } else {
                            to_add.push(((x - 1, y), dir));
                        }
                    }
                    Direction::Down => {
                        if y >= height - 2 {
                            to_add.push(((x, 1), dir));
                        } else {
                            to_add.push(((x, y + 1), dir));
                        }
                    }
                    Direction::Up => {
                        if y <= 1 {
                            to_add.push(((x, height - 2), dir));
                        } else {
                            to_add.push(((x, y - 1), dir));
                        }
                    }
                }
            }
        }
        for (k, v) in to_add {
            blizzards.entry(k).or_default().push(v);
        }
        for (x, y) in current.drain() {
            for (x, y) in [
                (x - 1, y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
                (x, y),
            ] {
                if (x, y) == start
                    || (0 < x
                    && 0 < y
                    && x < width - 1
                    && y < height - 1
                    && blizzards[&(x, y)].is_empty())
                {
                    next.insert((x, y));
                }
            }
        }
        if next.contains(&goal) {
            swap(current, next);
            return minute + 1;
        }
        swap(current, next);
        minute += 1;
    }
}