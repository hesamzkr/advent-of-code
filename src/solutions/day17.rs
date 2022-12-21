use std::collections::HashSet;

#[derive(Clone)]
struct Rock {
    shape: Vec<(i32, i32)>,
}

impl Rock {
    fn spawn(index: usize) -> Rock {
        vec![
            Rock {
                shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            Rock {
                shape: vec![(0, -1), (1, -1), (2, -1), (1, 0), (1, -2)],
            },
            Rock {
                shape: vec![(0, -2), (2, 0), (2, -1), (1, -2), (2, -2)],
            },
            Rock {
                shape: vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            },
            Rock {
                shape: vec![(0, 0), (1, 0), (0, -1), (1, -1)],
            },
        ][index]
            .clone()
    }
}

pub fn run(input: String) -> (usize, u32) {
    let input: Vec<char> = input.chars().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(jets: &Vec<char>) -> usize {
    let rocks: Vec<Rock> = Vec::new();

    let rock_index = 0;
    while rocks.len() < 2022 {
        let mut rock = Rock::spawn(rock_index % 5);
        let highest_point = 0;
        for i in 0..rock.shape.len() {
            rock.shape[i].0 += 3;
        }
    }

    0
}

fn part_two(jets: &Vec<char>) -> u32 {
    0
}
