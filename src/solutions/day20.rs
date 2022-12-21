pub fn run(input: String) -> (i64, i64) {
    let input: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &Vec<i64>) -> i64 {
    let mut indices: Vec<usize> = (0..input.len()).collect();

    for (i, num) in input.iter().enumerate() {
        let index = indices.iter().position(|index| index == &i).unwrap();
        indices.remove(index);
        let new_index = (index as i64 + num).rem_euclid(indices.len() as i64) as usize;
        indices.insert(new_index, i);
    }

    let initial_zero_index = input.iter().position(|x| *x == 0).unwrap();
    let zero_index = indices
        .iter()
        .position(|x| *x == initial_zero_index)
        .unwrap();

    [1000, 2000, 3000].iter().fold(0, |acc, i| {
        acc + input[indices[(zero_index + i).rem_euclid(input.len())]]
    })
}

fn part_two(input: &Vec<i64>) -> i64 {
    let input: Vec<i64> = input.iter().map(|x| x * 811589153).collect();
    let mut indices: Vec<usize> = (0..input.len()).collect();

    for _ in 0..10 {
        for (i, num) in input.iter().enumerate() {
            let index = indices.iter().position(|index| index == &i).unwrap();
            indices.remove(index);
            let new_index = (index as i64 + num).rem_euclid(indices.len() as i64) as usize;
            indices.insert(new_index, i);
        }
    }

    let initial_zero_index = input.iter().position(|x| *x == 0).unwrap();
    let zero_index = indices
        .iter()
        .position(|x| *x == initial_zero_index)
        .unwrap();

    [1000, 2000, 3000].iter().fold(0, |acc, i| {
        acc + input[indices[(zero_index + i).rem_euclid(input.len())]]
    })
}
