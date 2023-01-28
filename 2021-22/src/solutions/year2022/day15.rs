use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn run(input: String) -> (usize, i64) {
    let report: HashMap<Point, Point> = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| {
            let sensor_x = (x[2][2..]).replace(',', "").parse().unwrap();
            let sensor_y = (x[3][2..]).replace(':', "").parse().unwrap();
            let beacon_x = (x[8][2..]).replace(',', "").parse().unwrap();
            let beacon_y = (x[9][2..]).parse().unwrap();
            (
                Point {
                    x: sensor_x,
                    y: sensor_y,
                },
                Point {
                    x: beacon_x,
                    y: beacon_y,
                },
            )
        })
        .collect();

    let answer_one = part_one(&report);
    let answer_two = part_two(&report).unwrap();

    (answer_one, answer_two)
}

fn part_one(report: &HashMap<Point, Point>) -> usize {
    const Y: i64 = 2_000_000;

    let mut cant_contain: HashSet<Point> = HashSet::new();
    for (sensor, closest_beacon) in report {
        let distance = sensor.manhattan_distance(closest_beacon);
        if sensor.y + distance < Y || sensor.y - distance > Y {
            continue;
        }

        let max_x_offset = if sensor.y < Y {
            sensor.y + distance - Y
        } else {
            Y - (sensor.y - distance)
        };

        for x_offset in -max_x_offset..=max_x_offset {
            if (closest_beacon.x - sensor.x, closest_beacon.y - sensor.y)
                != (x_offset, Y - sensor.y)
            {
                cant_contain.insert(Point {
                    x: sensor.x + x_offset,
                    y: Y,
                });
            }
        }
    }

    cant_contain.len()
}

fn part_two(report: &HashMap<Point, Point>) -> Option<i64> {
    const MAX: i64 = 4_000_000;

    for y in 0..=MAX {
        let mut x = 0;
        while x <= MAX {
            let mut detected = false;
            for (sensor, closest_beacon) in report {
                let distance = sensor.manhattan_distance(closest_beacon);
                if sensor.y + distance < y || sensor.y - distance > y {
                    continue;
                }
                let max_x_offset = if sensor.y < y {
                    sensor.y + distance - y
                } else {
                    y - (sensor.y - distance)
                };
                if sensor.x - max_x_offset <= x && sensor.x + max_x_offset >= x {
                    x = sensor.x + max_x_offset + 1;
                    detected = true;
                    break;
                }
            }
            if !detected {
                return Some(4_000_000 * x + y);
            }
        }
    }

    None
}
