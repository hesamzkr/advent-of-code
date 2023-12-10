#[derive(Debug)]
struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

#[derive(Debug, Clone)]

struct Range {
    from: i64,
    to: i64,
}

pub fn run(input: String) -> (i64, i64) {
    let input: Vec<&str> = input.split("\n\n").collect();

    let seeds = input[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut parsed: Vec<Vec<Rule>> = vec![];
    for i in 1..input.len() {
        let converts: Vec<Rule> = input[i]
            .lines()
            .skip(1)
            .map(|line| {
                let numbers: Vec<i64> = line
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect();

                Rule {
                    destination: numbers[0],
                    source: numbers[1],
                    range: numbers[2],
                }
            })
            .collect();

        parsed.push(converts);
    }

    let answer_one = part_one(&seeds, &parsed);
    let answer_two = part_two(&seeds, &mut parsed);

    (answer_one, answer_two)
}

fn part_one(seeds: &Vec<i64>, maps: &Vec<Vec<Rule>>) -> i64 {
    let mut min = i64::MAX;

    for seed in seeds {
        let mut curr = *seed;

        for map in maps {
            for rule in map {
                let rule_applies = curr >= rule.source && curr <= rule.source + rule.range;
                if rule_applies {
                    let offset = curr - rule.source;
                    curr = rule.destination + offset;
                    break;
                }
            }
        }

        min = min.min(curr);
    }
    min
}

fn part_two(seeds: &Vec<i64>, maps: &mut Vec<Vec<Rule>>) -> i64 {
    let seeds = seeds.chunks(2).into_iter().map(|mut chunk| {
        let from = chunk[0];

        let range = chunk[1];

        Range {
            from,

            to: from + range,
        }
    });

    maps.iter_mut().for_each(|block| {
        block.sort_by(|a, b| a.source.cmp(&b.source));
    });

    let mut curr_ranges: Vec<Range> = seeds.collect();

    for map in maps {
        let mut new_ranges: Vec<Range> = Vec::new();

        for range in &curr_ranges {
            let mut curr = range.clone();

            for rule in &mut *map {
                let offset = rule.destination - rule.source;

                let rule_applies = curr.from <= curr.to
                    && curr.from <= rule.source + rule.range
                    && curr.to >= rule.source;

                if rule_applies {
                    if curr.from < rule.source {
                        new_ranges.push(Range {
                            from: curr.from,

                            to: rule.source - 1,
                        });

                        curr.from = rule.source;

                        if curr.to < rule.source + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,

                                to: curr.to + offset,
                            });

                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,

                                to: rule.source + rule.range - 1 + offset,
                            });

                            curr.from = rule.source + rule.range;
                        }
                    } else if curr.to < rule.source + rule.range {
                        new_ranges.push(Range {
                            from: curr.from + offset,

                            to: curr.to + offset,
                        });

                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,

                            to: rule.source + rule.range - 1 + offset,
                        });

                        curr.from = rule.source + rule.range;
                    }
                }
            }

            if curr.from <= curr.to {
                new_ranges.push(curr);
            }
        }

        curr_ranges = new_ranges;
    }

    curr_ranges.iter().map(|range| range.from).min().unwrap()
}
