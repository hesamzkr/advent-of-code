use std::cmp::max;
use std::cmp::min;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Valve {
    flow: i32,
    connections: Vec<String>,
}

pub fn run(input: String) -> (i32, i32) {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| {
            (
                x[1].to_owned(),
                Valve {
                    flow: x[4].replace("rate=", "").replace(';', "").parse().unwrap(),
                    connections: x[9..].iter().map(|y| y.replace(',', "")).collect(),
                },
            )
        })
        .collect();

    let edges = shortest_paths(&valves);

    let answer_one = part_one(&valves, &edges);
    let answer_two = part_two(&valves, &edges);

    (answer_one, answer_two)
}

fn part_one(valves: &HashMap<String, Valve>, edges: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut best_scores: HashMap<BTreeSet<String>, i32> = HashMap::new();
    let mut best_path = i32::MIN;
    let mut paths: Vec<(String, i32, i32, BTreeSet<String>)> =
        vec![("AA".to_owned(), 0, 0, BTreeSet::new())];

    while !paths.is_empty() {
        let (cursor, minutes, total_flow, visited) = paths.pop().unwrap();
        best_path = max(best_path, total_flow);
        for (target, distance) in edges.get(&cursor).unwrap() {
            let mut next_targets = visited.clone();
            next_targets.insert(target.clone());
            let next_total_flow =
                total_flow + valves.get(target).unwrap().flow * (30 - minutes - distance);

            if best_scores.get(&next_targets).unwrap_or(&i32::MIN) >= &next_total_flow {
                continue;
            }

            if !visited.contains(target) && distance + minutes < 30 {
                best_scores.insert(next_targets.clone(), next_total_flow);
                paths.push((
                    target.to_owned(),
                    distance + minutes,
                    next_total_flow,
                    next_targets.clone(),
                ))
            }
        }
    }

    best_path
}

fn part_two(valves: &HashMap<String, Valve>, edges: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut best_scores: HashMap<BTreeSet<String>, i32> = HashMap::new();
    let mut best_path = i32::MIN;
    let mut paths: Vec<(String, String, i32, i32, i32, BTreeSet<String>)> =
        vec![("AA".to_owned(), "AA".to_owned(), 0, 0, 0, BTreeSet::new())];

    while !paths.is_empty() {
        let (cursor_a, cursor_b, minutes_a, minutes_b, total_flow, visited) = paths.pop().unwrap();
        best_path = max(best_path, total_flow);
        for (target, distance) in edges.get(&cursor_a).unwrap() {
            let mut next_targets = visited.clone();
            next_targets.insert(target.clone());
            let next_total_flow =
                total_flow + valves.get(target).unwrap().flow * (26 - minutes_a - distance);

            if best_scores.get(&next_targets).unwrap_or(&i32::MIN) >= &next_total_flow {
                continue;
            }

            if !visited.contains(target) && distance + minutes_a < 26 {
                best_scores.insert(next_targets.clone(), next_total_flow);
                paths.push((
                    target.to_owned(),
                    cursor_b.clone(),
                    distance + minutes_a,
                    minutes_b,
                    next_total_flow,
                    next_targets.clone(),
                ))
            }
        }

        for (target, distance) in edges.get(&cursor_b).unwrap() {
            let mut next_targets = visited.clone();
            next_targets.insert(target.clone());
            let next_total_flow =
                total_flow + valves.get(target).unwrap().flow * (26 - minutes_b - distance);

            if best_scores.get(&next_targets).unwrap_or(&i32::MIN) >= &next_total_flow {
                continue;
            }

            if !visited.contains(target) && distance + minutes_b < 26 {
                best_scores.insert(next_targets.clone(), next_total_flow);
                paths.push((
                    cursor_a.clone(),
                    target.to_owned(),
                    minutes_a,
                    distance + minutes_b,
                    next_total_flow,
                    next_targets.clone(),
                ))
            }
        }
    }

    best_path
}

fn shortest_paths(valves: &HashMap<String, Valve>) -> HashMap<String, HashMap<String, i32>> {
    let mut result: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (name, valve) in valves {
        let mut temp: HashMap<String, i32> = HashMap::new();
        if valve.flow > 0 {
            temp.insert(name.clone(), 1);
        }
        result.insert(name.clone(), temp);
    }

    for _ in 0..valves.len() {
        for (name, valve) in valves {
            for connection in &valve.connections {
                if let Some(x) = result.clone().get(connection) {
                    for (trans_name, price) in x {
                        let result_name = &result[name].clone();
                        result.get_mut(name).unwrap().insert(
                            trans_name.clone(),
                            min(*result_name.get(trans_name).unwrap_or(&i32::MAX), price + 1),
                        );
                    }
                }
            }
        }
    }

    result
}
