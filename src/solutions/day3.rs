use crate::common::Solution;

pub struct Day3;

impl Solution for Day3 {
    type Parsed = Vec<String>;
    type PartOneOutput = u32;
    type PartTwoOutput = u32;

    fn parse(input: String) -> Self::Parsed {
        input.lines().map(|x| x.to_string()).collect()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        let mut sum = 0;
        for rucksack in data {
            let mid = rucksack.len() / 2;
            let first = rucksack.chars().take(mid);
            let second = rucksack.chars().skip(mid);
            let mut flag = true;
            for i in first {
                for j in rucksack.chars().skip(mid) {
                    if i == j && flag {
                        let mut x;
                        if i.is_lowercase() {
                            x = i as u32 - 96;
                        } else {
                            x = i as u32 - 38;
                        }
                        sum += x;
                        flag = false;
                    }
                }
            }
        }

        sum

    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        let mut sum = 0;
        for groupi in (0..data.len() - 2).step_by(3) {
            let r1 = data[groupi].chars();
            let r2 = data[groupi + 1].chars();
            let r3 = data[groupi + 2].chars();

            let mut shared_char: char = '8';
            for x in r1 {
                for y in data[groupi + 1].chars() {
                    if x == y {
                        for z in data[groupi + 2].chars() {
                            if x == z {
                                shared_char = x;
                            }
                        }
                    }
                }
            }

            let mut added_num = 0;
            if shared_char.is_lowercase() {
                added_num = shared_char as u32 - 96;
            } else {
                added_num = shared_char as u32 - 38;
            }
            // println!("{shared_char}, {added_num}");
            sum += added_num;
        }

        sum
    }
}

impl Day3 {
    fn char_to_num(c: char) -> u32 {
        if c.is_lowercase() {
            c as u32 - 96
        } else {
            c as u32 - 38
        }   
    }
}
