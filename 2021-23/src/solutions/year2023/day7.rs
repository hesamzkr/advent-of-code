use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
}

impl Type {
    fn parse_part_one(hand: &str) -> Type {
        let mut set = HashSet::new();
        for target in hand.chars() {
            let count = hand.chars().filter(|&c| c == target).count();
            set.insert((target, count));
        }

        let counts: Vec<usize> = set.iter().map(|(ch, count)| *count).collect();

        if counts.iter().filter(|count| **count == 5).next().is_some() {
            Type::FiveOfAKind
        } else if counts.iter().filter(|count| **count == 4).next().is_some() {
            Type::FourOfAKind
        } else if counts.iter().filter(|count| **count == 3).next().is_some() {
            if counts.iter().filter(|count| **count == 2).next().is_some() {
                Type::FullHouse
            } else {
                Type::ThreeOfAKind
            }
        } else {
            match counts.iter().filter(|count| **count == 2).count() {
                1 => Type::OnePair,
                2 => Type::TwoPair,
                0 => Type::HighCard,
                _ => panic!("Impossible"),
            }
        }
    }

    fn parse_part_two(hand: &str) -> Type {
        let mut set = HashSet::new();
        for target in hand.chars() {
            if target == 'J' {
                continue;
            }
            let count = hand.chars().filter(|&c| c == target).count();
            set.insert((target, count));
        }

        let j_count = hand.chars().filter(|&c| c == 'J').count();

        let counts: Vec<usize> = set.iter().map(|(ch, count)| *count).collect();

        if counts.iter().filter(|count| **count == 5).next().is_some() {
            Type::FiveOfAKind
        } else if counts.iter().filter(|count| **count == 4).next().is_some() {
            if j_count == 1 {
                Type::FiveOfAKind
            } else {
                Type::FourOfAKind
            }
        } else if counts.iter().filter(|count| **count == 3).next().is_some() {
            if counts.iter().filter(|count| **count == 2).next().is_some() {
                if j_count == 2 {
                    Type::FiveOfAKind
                } else {
                    Type::FullHouse
                }
            } else {
                if j_count == 2 {
                    Type::FiveOfAKind
                } else if j_count == 1 {
                    Type::FourOfAKind
                } else {
                    Type::ThreeOfAKind
                }
            }
        } else {
            match counts.iter().filter(|count| **count == 2).count() {
                1 => {
                    if j_count == 3 {
                        Type::FiveOfAKind
                    } else if j_count == 2 {
                        Type::FourOfAKind
                    } else if j_count == 1 {
                        Type::ThreeOfAKind
                    } else {
                        Type::OnePair
                    }
                }
                2 => {
                    if j_count == 1 {
                        Type::FullHouse
                    } else {
                        Type::TwoPair
                    }
                }
                0 => match j_count {
                    0 => Type::HighCard,
                    1 => Type::OnePair,
                    2 => Type::ThreeOfAKind,
                    3 => Type::FourOfAKind,
                    4 => Type::FiveOfAKind,
                    5 => Type::FiveOfAKind,
                    _ => panic!("Impossible"),
                },
                _ => panic!("Impossible"),
            }
        }
    }
}

pub fn run(input: String) -> (usize, usize) {
    let input: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            Hand {
                cards: split.next().unwrap(),
                bid: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(hands: &Vec<Hand>) -> usize {
    let mut parsed_hands = vec![];
    for hand in hands {
        let win_type = Type::parse_part_one(hand.cards);
        parsed_hands.push((hand, win_type));
    }

    parsed_hands.sort_by(|a, b| custom_sort(a, b, "AKQJT98765432"));

    parsed_hands.reverse();

    parsed_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (hand, _))| acc + (i + 1) * hand.bid)
}

fn part_two(hands: &Vec<Hand>) -> usize {
    let mut parsed_hands = vec![];
    for hand in hands {
        let win_type = Type::parse_part_two(hand.cards);
        parsed_hands.push((hand, win_type));
    }

    parsed_hands.sort_by(|a, b| custom_sort(a, b, "AKQT98765432J"));

    parsed_hands.reverse();

    parsed_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (hand, _))| acc + (i + 1) * hand.bid)
}

fn custom_sort(a: &(&Hand, Type), b: &(&Hand, Type), string_order: &str) -> Ordering {
    let type_order = a.1.cmp(&b.1);

    if type_order != Ordering::Equal {
        return type_order;
    }

    for i in 0..a.0.cards.chars().count() {
        let type_order = string_order
            .find(a.0.cards.chars().collect::<Vec<char>>()[i])
            .unwrap()
            .cmp(
                &string_order
                    .find(b.0.cards.chars().collect::<Vec<char>>()[i])
                    .unwrap(),
            );

        if type_order != Ordering::Equal {
            return type_order;
        }
    }

    Ordering::Equal
}
