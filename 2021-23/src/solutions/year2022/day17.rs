use std::collections::{HashSet, HashMap};

type Point = (i64, i64);

#[derive(Clone)]
struct Rock {
    positions: Vec<Point>
}

impl Rock {
    fn spawn(&self, highest_y: i64) -> Rock {
        let mut rock = self.clone();
        for i in 0..self.positions.len() {
            rock.positions[i].1 += highest_y + 4;
        }
        rock
    }

    fn move_right(&mut self, map: &HashSet<Point>) {
        let mut new_position = vec![];
        for i in 0..self.positions.len() {
            let mut position = self.positions[i];
            position.0 += 1;
            if position.0 > 3 || map.contains(&position) {
                return;
            }
         
            new_position.push(position);
        }
        self.positions = new_position;
    }

    fn move_left(&mut self, map: &HashSet<Point>) {
        let mut new_position = vec![];
        for i in 0..self.positions.len() {
            let mut position = self.positions[i];
            position.0 -= 1;
            if position.0 < -3 || map.contains(&position) {
                return;
            }
         
            new_position.push(position);
        }
        self.positions = new_position;
    }

    fn move_down(&mut self, map: &HashSet<Point>) -> bool {
        let mut new_position = vec![];
        for i in 0..self.positions.len() {
            let mut position = self.positions[i];
            position.1 -= 1;
            if position.1 < 1 || map.contains(&position) {
                return false;
            }
            new_position.push(position);
        }
        self.positions = new_position;
        true
    }
}

pub fn run(input: String) -> (i64, i64) {
    let input: Vec<char> = input.chars().collect();

    let rocks: [Rock; 5] = [  
        Rock { positions: vec![(-1, 0), (0, 0), (1, 0), (2, 0)] },
        Rock { positions: vec![(0, 0), (-1, 1), (0, 1), (1, 1), (0, 2)] },
        Rock { positions: vec![(-1, 0), (0, 0), (1, 0), (1, 1), (1, 2)] },
        Rock { positions: vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)] },
        Rock { positions: vec![(-1, 0), (0, 0), (-1, 1), (0, 1)] }
    ];

    let answer_one = part_one(&input, &rocks);
    let answer_two = part_two(&input, &rocks);

    (answer_one, answer_two)
}

fn part_one(jets: &Vec<char>, rock_shapes: &[Rock]) -> i64 {
    let mut rock_index = 0;
    let mut landed_rocks = 0;
    let mut jets_index = 0;
    let mut highest_y = 0;
    let mut rock = rock_shapes[0].spawn(highest_y);
    let mut map: HashSet<Point> = HashSet::new();

    while landed_rocks < 2022 {
        match jets[jets_index] {
            '<' => rock.move_left(&map),
            '>' => rock.move_right(&map),
            _ => panic!("Invalid character"),
        }

        if !rock.move_down(&map) {
            for pos in &rock.positions {
                if pos.1 > highest_y {
                    highest_y = pos.1;
                }
                map.insert(*pos);
            }
            landed_rocks += 1;
            rock_index = (rock_index + 1) % 5;
            rock = rock_shapes[rock_index].spawn(highest_y);
        }

        jets_index = (jets_index + 1) % jets.len();
    }


    highest_y
}

fn part_two(jets: &Vec<char>, rock_shapes: &[Rock]) -> i64 {
    let mut rock_index = 0;
    let mut landed_rocks: i64 = 0;
    let mut jets_index = 0;
    let mut highest_y = 0;
    let mut rock = rock_shapes[0].spawn(highest_y);
    let mut map: HashSet<Point> = HashSet::new();
    let mut potential_cycles: HashMap<(usize, usize, Vec<u8>), (i64, i64)> = HashMap::new();
    let mut cycle = (0, 0);
    let mut cycle_found = false;

    loop {
        match jets[jets_index] {
            '<' => rock.move_left(&map),
            '>' => rock.move_right(&map),
            _ => panic!("Invalid character"),
        }

        if !rock.move_down(&map) {
            for pos in &rock.positions {
                if pos.1 > highest_y {
                    highest_y = pos.1;
                }
                map.insert(*pos);
            }
            landed_rocks += 1;

            let bottom_row: Vec<u8> = (highest_y-30..=highest_y).map(|y|
                (-3..=3).fold(0,
                    |acc, x| (acc << 1) + (map.contains(&(x, y)) as u8)
                )
            ).collect();

            let indices = (jets_index, rock_index, bottom_row);
            if !cycle_found && potential_cycles.contains_key(&indices) {
                let cycle_info = potential_cycles.get(&indices).unwrap();
                cycle = (highest_y - cycle_info.0, landed_rocks - cycle_info.1);
                cycle_found = true;
                if (1_000_000_000_000 - landed_rocks) % cycle.1 == 0 {
                    break;
                }
            } else if cycle_found && (1_000_000_000_000 - landed_rocks) % cycle.1 == 0 {
                break;
            } else {
                potential_cycles.insert(indices, (highest_y, landed_rocks));
            }

            rock_index = (rock_index + 1) % 5;
            rock = rock_shapes[rock_index].spawn(highest_y);
        }

        jets_index = (jets_index + 1) % jets.len();
    }


    highest_y + cycle.0 * ((1_000_000_000_000 - landed_rocks) / cycle.1)
}
