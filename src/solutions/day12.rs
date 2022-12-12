use std::collections::{HashSet, VecDeque};

pub fn run(input: String) -> (usize, usize) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let answer_one = part_one(&mut grid);
    let answer_two = part_two(&grid);

    (answer_one, answer_two)
}

fn part_one(grid: &mut Vec<Vec<char>>) -> usize {
    let mut start_pos = (0, 0);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start_pos = (i, j);
                grid[i][j] = 'a';
                break 'outer;
            }
        }
    }

    shortest_distance(grid, start_pos).unwrap()
}

fn part_two(grid: &Vec<Vec<char>>) -> usize {
    let mut start_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: Vec<usize> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'a' {
                start_positions.insert((i, j));
            }
        }
    }

    for start_pos in start_positions {
        if let Some(distance) = shortest_distance(grid, start_pos) {
            distances.push(distance);
        }
    }

    *distances.iter().min().unwrap()
}

fn shortest_distance(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
    visited.insert(start_pos);
    paths.push_back(vec![start_pos]);

    'outer: while !paths.is_empty() {
        let path = paths[0].clone();
        let last_node = path.last().unwrap();
        let last_node_value = grid[last_node.0][last_node.1];

        for (x, y) in &[(2, 1), (0, 1), (1, 2), (1, 0)] {
            let x: i32 = x - 1;
            let y: i32 = y - 1;
            let other_node_coord = (last_node.0 as i32 + x, last_node.1 as i32 + y);
            if other_node_coord.0 < 0
                || other_node_coord.0 >= grid.len() as i32
                || other_node_coord.1 < 0
                || other_node_coord.1 >= grid[0].len() as i32
            {
                continue;
            }

            let other_node_coord = (other_node_coord.0 as usize, other_node_coord.1 as usize);
            let other_node = grid[other_node_coord.0][other_node_coord.1];

            if !visited.contains(&other_node_coord)
                && (other_node as u32 <= last_node_value as u32 + 1 || other_node == 'E')
            {
                visited.insert(other_node_coord);
                let mut temp = path.clone();
                temp.push(other_node_coord);
                paths.push_back(temp);
                if other_node == 'E' {
                    break 'outer;
                }
            }
        }
        paths.pop_front();
    }

    paths.pop_back().map(|path| path.len() - 1)
}
