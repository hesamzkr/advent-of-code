#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: (String, String),
    test: u64,
    if_true: usize,
    if_false: usize,
}

pub fn run(input: String) -> (u64, u64) {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|x| x.lines())
        .map(|mut x| {
            x.next();
            let items = x
                .next()
                .unwrap()
                .replace("  Starting items: ", "")
                .replace(" ", "")
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            let replaced = x.next().unwrap().replace("  Operation: new = old ", "");
            let mut operation_split = replaced.split_whitespace();
            let operation = (
                operation_split.next().unwrap().to_string(),
                operation_split.next().unwrap().to_string(),
            );
            let test = x
                .next()
                .unwrap()
                .replace("  Test: divisible by ", "")
                .parse()
                .unwrap();
            let if_true = x
                .next()
                .unwrap()
                .replace("    If true: throw to monkey ", "")
                .parse()
                .unwrap();
            let if_false = x
                .next()
                .unwrap()
                .replace("    If false: throw to monkey ", "")
                .parse()
                .unwrap();

            Monkey {
                items: items,
                operation: operation,
                test: test,
                if_true: if_true,
                if_false: if_false,
            }
        })
        .collect();

    let answer_one = part_one(monkeys.clone());
    let answer_two = part_two(monkeys.clone());

    (answer_one, answer_two)
}

fn part_one(mut monkeys: Vec<Monkey>) -> u64 {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items[0];
                let operand = match monkey.operation.1.parse() {
                    Ok(x) => x,
                    Err(_) => item,
                };

                let test_by = match monkey.operation.0.as_str() {
                    "*" => item * operand,
                    "+" => item + operand,
                    _ => panic!("Invalid operator"),
                } / 3;

                if test_by % monkey.test == 0 {
                    monkeys[monkey.if_true].items.push(test_by);
                } else {
                    monkeys[monkey.if_false].items.push(test_by);
                }

                monkeys[i].items.remove(0);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn part_two(mut monkeys: Vec<Monkey>) -> u64 {
    let mut inspections = vec![0; monkeys.len()];
    let mut modulo = 1;
    for monkey in &monkeys {
        modulo = lcm(modulo, monkey.test);
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items[0];
                let operand = match monkey.operation.1.parse() {
                    Ok(x) => x,
                    Err(_) => item,
                };

                let test_by = match monkey.operation.0.as_str() {
                    "*" => item * operand,
                    "+" => item + operand,
                    _ => panic!("Invalid operator"),
                } % modulo;

                if test_by % monkey.test == 0 {
                    monkeys[monkey.if_true].items.push(test_by);
                } else {
                    monkeys[monkey.if_false].items.push(test_by);
                }

                monkeys[i].items.remove(0);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn lcm(a: u64, b: u64) -> u64 {
    let mut greater = if a > b { a } else { b };

    loop {
        if greater % a == 0 && greater % b == 0 {
            return greater;
        }
        greater += 1;
    }
}
