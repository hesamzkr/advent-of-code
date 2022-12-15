use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn run(input: String) -> (u64, u32) {
    let report: HashMap<Point, Point> = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| {
            let sensor_x = (&x[2][2..]).replace(",", "").parse().unwrap();
            let sensor_y = (&x[3][2..]).replace(":", "").parse().unwrap();
            let beacon_x = (&x[8][2..]).replace(",", "").parse().unwrap();
            let beacon_y = (&x[9][2..]).parse().unwrap();
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
    let answer_two = part_two(&report);

    (answer_one, answer_two)
}

fn part_one(report: &HashMap<Point, Point>) -> u64 {
    let y = 2_000_000;
    // let y = 10;
    let max_x = report
        .iter()
        .map(|(sensor, beacon)| beacon.x)
        .max()
        .unwrap();
    let min_x = report
        .iter()
        .map(|(sensor, beacon)| beacon.x)
        .min()
        .unwrap();

    let mut set: HashSet<Point> = HashSet::new();
    for (sensor, closest_beacon) in report {
        let distance = sensor.manhattan_distance(closest_beacon) as i64;
        if sensor.y + distance < y || sensor.y - distance > y {
            continue;
        }

        let max_x_offset = if sensor.y < y {
            sensor.y + distance - y
        } else {
            y - (sensor.y - distance)
        };

        for x_offset in -max_x_offset..=max_x_offset {
            if (closest_beacon.x - sensor.x, closest_beacon.y - sensor.y)
                != (x_offset, y - sensor.y)
            {
                set.insert(Point {
                    x: sensor.x + x_offset,
                    y,
                });
            }
        }
    }

    set.len() as u64
    // let mut count = 0;
    // for x in min_x..=max_x {
    //     let current_point = Point { x, y };
    //     for (sensor, closest_beacon) in report {
    //         let distance_to_point = sensor.manhattan_distance(&current_point);
    //         let closest_distance = sensor.manhattan_distance(closest_beacon);
    //         if distance_to_point <= closest_distance && current_point != *closest_beacon {
    //             count += 1;
    //             break;
    //         }
    //     }
    // }

    // count
}

fn part_two(input: &HashMap<Point, Point>) -> u32 {
    0
}
