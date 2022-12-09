use std::collections::HashSet;

struct Motion {
    direction: String,
    steps: i32,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run(input: String) -> (usize, usize) {
    let input: Vec<Motion> = input
        .lines()
        .map(|x| x.split_whitespace())
        .map(|mut x| Motion {
            direction: x.next().unwrap().to_string(),
            steps: x.next().unwrap().parse().unwrap(),
        })
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<Motion>) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited_points: HashSet<Point> = HashSet::new();
    for motion in input {
        let dir = match motion.direction.as_str() {
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            _ => panic!("Invalid direction"),
        };

        for _ in 0..motion.steps {
            head.x += dir.x;
            head.y += dir.y;
            let diff = Point {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                if diff.x == 0 {
                    tail.y += diff.y / diff.y.abs();
                } else if diff.y == 0 {
                    tail.x += diff.x / diff.x.abs();
                } else {
                    tail.y += diff.y / diff.y.abs();
                    tail.x += diff.x / diff.x.abs();
                }
            }

            visited_points.insert(tail.clone());
        }
    }

    visited_points.len()
}

fn part_two(input: &Vec<Motion>) -> usize {
    let mut rope: Vec<Point> = (0..10).map(|_| Point { x: 0, y: 0 }).collect();
    let mut visited_points: HashSet<Point> = HashSet::new();
    for motion in input {
        let dir = match motion.direction.as_str() {
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            _ => panic!("Invalid direction"),
        };

        for x in 0..motion.steps {
            rope[0].x += dir.x;
            rope[0].y += dir.y;
            for i in 0..rope.len() - 1 {
                let head = rope[i];
                let tail = rope[i + 1];
                let diff = Point {
                    x: head.x - tail.x,
                    y: head.y - tail.y,
                };
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    if diff.x == 0 {
                        rope[i + 1].y += diff.y / diff.y.abs();
                    } else if diff.y == 0 {
                        rope[i + 1].x += diff.x / diff.x.abs();
                    } else {
                        rope[i + 1].y += diff.y / diff.y.abs();
                        rope[i + 1].x += diff.x / diff.x.abs();
                    }
                }

                if i + 1 == rope.len() - 1 {
                    visited_points.insert(rope[i + 1]);
                }
            }
        }
    }

    visited_points.len()
}
