pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| line.chars().filter(|char| char.is_numeric()))
        .map(|mut numbers| {
            let last = numbers.clone().last().unwrap();
            format!("{}{}", numbers.next().unwrap(), last)
        })
        .map(|number| number.parse::<u32>().unwrap())
        .sum()
}

fn part_two(lines: &[&str]) -> u32 {
    let word_numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    lines
        .iter()
        .map(|line| {
            let v1 = line.chars().enumerate().filter_map(|(i, char)| {
                if char.is_numeric() {
                    Some((char.to_string(), i))
                } else {
                    None
                }
            });

            let v2 = word_numbers.iter().enumerate().flat_map(|(i, word)| {
                line.match_indices(word)
                    .map(move |(index, _)| (format!("{}", i + 1), index))
            });

            v1.chain(v2).collect::<Vec<(String, usize)>>()
        })
        .map(|mut numbers| {
            numbers.sort_by(|a, b| a.1.cmp(&b.1));
            let number = format!(
                "{}{}",
                numbers.first().unwrap().0,
                numbers.last().unwrap().0
            );
            number.parse::<u32>().unwrap()
        })
        .sum()
}
