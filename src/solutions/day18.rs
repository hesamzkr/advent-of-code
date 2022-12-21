use std::collections::{HashSet, VecDeque};

pub fn run(input: String) -> (usize, usize) {
    let cubes: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect())
        .collect();

    let answer_one = part_one(&cubes);
    let answer_two = part_two(&cubes);

    (answer_one, answer_two)
}

fn part_one(cubes: &Vec<Vec<i32>>) -> usize {
    let mut surface_area = 0;
    for cube in cubes {
        let mut open_surface = 6;
        for i in 0..3 {
            for j in [-1, 1] {
                let mut adjacent_cube = cube.clone();
                adjacent_cube[i] += j;
                if cubes.contains(&adjacent_cube) {
                    open_surface -= 1;
                }
            }
        }
        surface_area += open_surface;
    }

    surface_area
}

fn part_two(cubes: &[Vec<i32>]) -> usize {
    let mut air_cubes: HashSet<Vec<i32>> = HashSet::new();
    let mut frontier: VecDeque<Vec<i32>> = VecDeque::new();
    let mut surface_area = 0;

    let x_min = cubes.iter().map(|x| x[0]).min().unwrap() - 1;
    let y_min = cubes.iter().map(|x| x[1]).min().unwrap() - 1;
    let z_min = cubes.iter().map(|x| x[2]).min().unwrap() - 1;
    let x_max = cubes.iter().map(|x| x[0]).max().unwrap() + 1;
    let y_max = cubes.iter().map(|x| x[1]).max().unwrap() + 1;
    let z_max = cubes.iter().map(|x| x[2]).max().unwrap() + 1;

    for z in z_min..=z_max {
        for x in x_min..=x_max {
            frontier.push_back(vec![x, y_min, z]);
            air_cubes.insert(vec![x, y_min, z]);
        }
    }

    while !frontier.is_empty() {
        let cube = frontier.pop_front().unwrap();
        for i in 0..3 {
            for j in [-1, 1] {
                let mut adjacent_cube = cube.clone();
                adjacent_cube[i] += j;
                if (x_min..=x_max).contains(&adjacent_cube[0])
                    && (y_min..=y_max).contains(&adjacent_cube[1])
                    && (z_min..=z_max).contains(&adjacent_cube[2])
                {
                    if cubes.contains(&adjacent_cube) {
                        surface_area += 1;
                    } else if air_cubes.insert(adjacent_cube.clone()) {
                        frontier.push_back(adjacent_cube);
                    }
                }
            }
        }
    }

    surface_area
}
