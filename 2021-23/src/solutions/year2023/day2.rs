#[derive(Copy, Clone, PartialEq)]
enum Color {
    Red = 12,
    Green = 13,
    Blue = 14,
}

struct Pick {
    number: u32,
    color: Color,
}

pub fn run(input: String) -> (usize, u32) {
    let input = input
        .lines()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split(';')
                .map(|picks| {
                    picks
                        .split(',')
                        .map(|pick| {
                            let x = pick.split_whitespace().collect::<Vec<&str>>();
                            Pick {
                                number: x[0].parse::<u32>().unwrap(),
                                color: match x[1] {
                                    "red" => Color::Red,
                                    "green" => Color::Green,
                                    "blue" => Color::Blue,
                                    _ => panic!("Unknown Color"),
                                },
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(games: &Vec<Vec<Vec<Pick>>>) -> usize {
    games
        .iter()
        .enumerate()
        .filter_map(|(i, game)| {
            if game
                .iter()
                .any(|round| round.iter().any(|pick| pick.number > (pick.color as u32)))
            {
                None
            } else {
                Some(i + 1)
            }
        })
        .sum()
}

fn part_two(games: &Vec<Vec<Vec<Pick>>>) -> u32 {
    let colors = [Color::Red, Color::Green, Color::Blue];
    games
        .iter()
        .map(|game| {
            colors
                .iter()
                .map(|color| {
                    game.iter()
                        .flat_map(|round| {
                            round.iter().filter_map(|pick| {
                                if pick.color == *color {
                                    Some(pick.number)
                                } else {
                                    None
                                }
                            })
                        })
                        .max()
                        .unwrap()
                })
                .fold(1, |acc, x| acc * x)
        })
        .sum()
}
