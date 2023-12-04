use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Card {
    number: usize,
    winning: Vec<u32>,
    yours: Vec<u32>,
}

pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Card> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let sections: Vec<Vec<u32>> = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .map(|section| {
                    section
                        .split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect();

            Card {
                number: i,
                winning: sections[0].clone(),
                yours: sections[1].clone(),
            }
        })
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            card.yours
                .iter()
                .filter(|number| card.winning.contains(number))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

fn part_two(cards: &Vec<Card>) -> u32 {
    let mut cards_map: HashMap<usize, u32> = cards.iter().map(|card| (card.number, 1)).collect();

    for (card_number, card) in cards.iter().enumerate() {
        let matches = card
            .yours
            .iter()
            .filter(|number| card.winning.contains(number))
            .count();

        for i in 1..=matches {
            let card_check = cards
                .iter()
                .filter(|card| card.number == card_number + i)
                .next();
            if let Some(next_card) = card_check {
                let how_many = *cards_map.get(&card_number).unwrap();
                cards_map
                    .entry(card_number + i)
                    .and_modify(|value| *value += how_many);
            }
        }
    }

    cards_map.iter().map(|(_, count)| count).sum()
}
