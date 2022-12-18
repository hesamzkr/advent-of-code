use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn add(&self, other: &Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn opposite(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn check_surrounding(&self, cubes: &Vec<Vector>) -> Vec<Vector> {
        let dirs: Vec<Vector> = vec![
            Vector { x: 1, y: 0, z: 0 },
            Vector { x: 0, y: 1, z: 0 },
            Vector { x: 0, y: 0, z: 1 },
        ];
        let mut surrounding_cubes: Vec<Vector> = Vec::new();

        for dir in &dirs {
            if cubes.contains(&self.add(dir)) {
                surrounding_cubes.push(self.add(dir));
            }
            if cubes.contains(&self.add(&dir.opposite())) {
                surrounding_cubes.push(self.add(&dir.opposite()));
            }
        }

        surrounding_cubes
    }
}

pub fn run(input: String) -> (usize, usize) {
    let cubes: Vec<Vector> = input
        .lines()
        .map(|x| {
            let mut x = x.split(',');
            Vector {
                x: x.next().unwrap().parse().unwrap(),
                y: x.next().unwrap().parse().unwrap(),
                z: x.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let answer_one = part_one(&cubes);
    let answer_two = part_two(&cubes);

    (answer_one, answer_two)
}

fn part_one(cubes: &Vec<Vector>) -> usize {
    let mut surface_area = 0;
    for cube in cubes {
        surface_area += 6 - cube.check_surrounding(cubes).len();
    }

    surface_area
}

fn part_two(cubes: &Vec<Vector>) -> usize {
    let dirs: Vec<Vector> = vec![
        Vector { x: 1, y: 0, z: 0 },
        Vector { x: 0, y: 1, z: 0 },
        Vector { x: 0, y: 0, z: 1 },
    ];
    let mut air_cubes: HashSet<Vector> = HashSet::new();

    'outer: for cube in cubes {
        for dir in &dirs {
            if !cubes.contains(&cube.add(dir)) {
                if cube.add(dir).check_surrounding(cubes).len() == 6 {
                    air_cubes.insert(cube.add(dir));
                }
            }
            if !cubes.contains(&cube.add(&dir.opposite())) {
                if cube.add(&dir.opposite()).check_surrounding(cubes).len() == 6 {
                    air_cubes.insert(cube.add(&dir.opposite()));
                }
            }
        }
    }

    println!("{:?}", air_cubes);
    part_one(cubes) - (air_cubes.len() * 6)
}
