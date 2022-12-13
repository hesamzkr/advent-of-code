use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].partial_cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.partial_cmp(&vec![Packet::Int(*b)]),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn run(input: String) -> (usize, usize) {
    let packets: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|x| x.lines())
        .map(|mut x| {
            let (left, _) = build_packet(x.next().unwrap());
            let (right, _) = build_packet(x.next().unwrap());
            (left, right)
        })
        .collect();

    let answer_one = part_one(&packets);
    let answer_two = part_two(&packets);

    (answer_one, answer_two)
}

fn part_one(packets: &[(Packet, Packet)]) -> usize {
    packets.iter().enumerate().fold(
        0,
        |acc, (i, (left, right))| if left < right { acc + i + 1 } else { acc },
    )
}

fn part_two(packets: &[(Packet, Packet)]) -> usize {
    let mut new_packets: Vec<&Packet> = Vec::new();
    for (left, right) in packets {
        new_packets.push(left);
        new_packets.push(right);
    }

    let dividers = vec![
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];

    new_packets.push(&dividers[0]);
    new_packets.push(&dividers[1]);

    new_packets.sort_unstable();

    new_packets.iter().enumerate().fold(1, |acc, (i, packet)| {
        if dividers.contains(packet) {
            acc * (i + 1)
        } else {
            acc
        }
    })
}

fn build_packet(line: &str) -> (Packet, usize) {
    let mut packets = Vec::new();
    let mut skip = 0;
    let mut number = String::new();
    for (i, c) in line.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            ']' => {
                if !number.is_empty() {
                    packets.push(Packet::Int(number.parse().unwrap()));
                }
                return (Packet::List(packets), i + 1);
            }
            '[' => {
                let (packet, to_skip) = build_packet(&line[i + 1..]);
                packets.push(packet);
                skip = to_skip;
            }
            ',' => {
                if !number.is_empty() {
                    packets.push(Packet::Int(number.parse().unwrap()));
                    number.clear();
                }
            }
            _ => number.push(c),
        }
    }

    (Packet::List(packets), line.len())
}
