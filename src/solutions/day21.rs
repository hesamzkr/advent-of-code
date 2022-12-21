use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Calculation {
    Value(i64),
    Op(Operation),
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    op: Operator,
}

impl Calculation {
    fn calculate(&self, monkeys: &HashMap<String, Calculation>) -> i64 {
        match self {
            Self::Value(x) => *x,
            Self::Op(operation) => {
                let lhs = monkeys.get(&operation.lhs).unwrap().calculate(monkeys);
                let rhs = monkeys.get(&operation.rhs).unwrap().calculate(monkeys);
                match operation.op {
                    Operator::Add => lhs + rhs,
                    Operator::Subtract => lhs - rhs,
                    Operator::Multiply => lhs * rhs,
                    Operator::Divide => lhs / rhs,
                }
            }
        }
    }

    fn get_lhs(&self, monkeys: &HashMap<String, Calculation>) -> Option<Calculation> {
        match self {
            Self::Value(_) => None,
            Self::Op(x) => Some(monkeys.get(&x.lhs).unwrap().clone()),
        }
    }

    fn get_rhs(&self, monkeys: &HashMap<String, Calculation>) -> Option<Calculation> {
        match self {
            Self::Value(_) => None,
            Self::Op(x) => Some(monkeys.get(&x.rhs).unwrap().clone()),
        }
    }
}

impl Operation {
    fn find_humn(
        &self,
        path: &[char],
        monkeys: &HashMap<String, Calculation>,
    ) -> Option<Vec<char>> {
        if self.lhs == *"humn" {
            let mut path = path.to_owned();
            path.push('l');
            return Some(path);
        } else if self.rhs == *"humn" {
            let mut path = path.to_owned();
            path.push('r');
            return Some(path);
        }

        let lhs_monkey = monkeys.get(&self.lhs).unwrap();
        let rhs_monkey = monkeys.get(&self.rhs).unwrap();
        for (direction, monkey) in [('l', lhs_monkey), ('r', rhs_monkey)] {
            let val = match monkey {
                Calculation::Value(_) => None,
                Calculation::Op(operation) => {
                    let mut path = path.to_owned();
                    path.push(direction);
                    operation.find_humn(&path, monkeys)
                }
            };

            if val.is_some() {
                return val;
            }
        }

        None
    }
}

pub fn run(input: String) -> (i64, i64) {
    let monkeys: HashMap<String, Calculation> = input
        .lines()
        .map(|line| {
            let calculation = match line.split(": ").nth(1).unwrap().parse::<i64>() {
                Ok(x) => Calculation::Value(x),
                Err(_) => {
                    let split: Vec<String> = line[6..]
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect();
                    Calculation::Op(Operation {
                        lhs: split[0].clone(),
                        rhs: split[2].clone(),
                        op: match split[1].as_str() {
                            "+" => Operator::Add,
                            "-" => Operator::Subtract,
                            "*" => Operator::Multiply,
                            "/" => Operator::Divide,
                            _ => panic!("Invalid operator"),
                        },
                    })
                }
            };
            (line[0..4].to_owned(), calculation)
        })
        .collect();

    let answer_one = part_one(&monkeys);
    let answer_two = part_two(&monkeys);

    (answer_one, answer_two)
}

fn part_one(monkeys: &HashMap<String, Calculation>) -> i64 {
    monkeys.get(&"root".to_string()).unwrap().calculate(monkeys)
}

fn part_two(monkeys: &HashMap<String, Calculation>) -> i64 {
    let monkey = monkeys.get(&"root".to_string()).unwrap();
    let path = match monkey {
        Calculation::Value(_) => panic!("Not an Operation"),
        Calculation::Op(x) => x.find_humn(&[], monkeys).unwrap(),
    };

    let (mut monkey, mut answer) = if path[0] == 'l' {
        (
            monkey.get_lhs(monkeys).unwrap(),
            monkey.get_rhs(monkeys).unwrap().calculate(monkeys),
        )
    } else {
        (
            monkey.get_rhs(monkeys).unwrap(),
            monkey.get_lhs(monkeys).unwrap().calculate(monkeys),
        )
    };

    for &direction in &path[1..] {
        let operation = match &monkey {
            Calculation::Value(_) => panic!("Not an Operation"),
            Calculation::Op(x) => x.clone(),
        };

        let lhs = monkey.get_lhs(monkeys).unwrap();
        let rhs = monkey.get_rhs(monkeys).unwrap();
        let val = if direction == 'l' {
            rhs.calculate(monkeys)
        } else {
            lhs.calculate(monkeys)
        };

        answer = match operation.op {
            Operator::Add => answer - val,
            Operator::Subtract => {
                if direction == 'l' {
                    answer + val
                } else {
                    val - answer
                }
            }
            Operator::Multiply => answer / val,
            Operator::Divide => {
                if direction == 'l' {
                    answer * val
                } else {
                    val / answer
                }
            }
        };

        monkey = if direction == 'l' {
            monkey.get_lhs(monkeys).unwrap()
        } else {
            rhs
        }
    }

    answer
}
