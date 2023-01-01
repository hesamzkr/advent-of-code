#[derive(Debug)]
struct Blueprint {
    ore_ore: u32,
    clay_ore: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32,
}

impl Blueprint {
    fn can_buy(&self, materials: &Materials, robot: u32) -> bool {
        match robot {
            0 if materials.ore >= self.ore_ore => true,
            1 if materials.ore >= self.clay_ore => true,
            2 if materials.ore >= self.obsidian_ore && materials.clay >= self.obsidian_clay => true,
            3 if materials.ore >= self.geode_ore && materials.obsidian >= self.geode_obsidian => true,
            _ => false,
        }
    }

    fn buy(&self, materials: &Materials, robots: &Robots, robot: u32) -> Option<(Materials, Robots)> {
        if !self.can_buy(materials, robot) {
            return None;
        }

        let mut materials = materials.clone();
        let mut robots = robots.clone();

        match robot {
            0 => {
                materials.ore -= self.ore_ore;
                robots.ore += 1;
            },
            1 => {
                materials.ore -= self.clay_ore;
                robots.clay += 1;
            },
            2 => {
                materials.ore -= self.obsidian_ore;
                materials.clay -= self.obsidian_clay;
                robots.obsidian += 1;
            },
            3 => {
                materials.ore -= self.geode_ore;
                materials.obsidian -= self.geode_obsidian;
                robots.geode += 1;
            },
            _ => return None,
        };

        Some((materials, robots))
    }

    fn get_ores(&self) -> [u32; 4] {
        [self.ore_ore, self.clay_ore, self.obsidian_ore, self.geode_ore]
    }
}

#[derive(Clone)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Robots {
    fn generate(&self, materials: &mut Materials) {
        materials.ore += self.ore;
        materials.clay += self.clay;
        materials.obsidian += self.obsidian;
        materials.geode += self.geode;
    }
}

#[derive(Clone)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

pub fn run(input: String) -> (u32, u32) {
    let blueprints: Vec<Blueprint> = input
    .lines()
    .map(|line| {
        let line_split: Vec<&str> = line.split_whitespace().collect();

        let ore_ore = line_split[6].parse().unwrap();
        let clay_ore = line_split[12].parse().unwrap();
        let obsidian_ore = line_split[18].parse().unwrap();
        let obsidian_clay = line_split[21].parse().unwrap();
        let geode_ore = line_split[27].parse().unwrap();
        let geode_obsidian = line_split[30].parse().unwrap();

        Blueprint {
            ore_ore,
            clay_ore,
            obsidian_ore,
            obsidian_clay,
            geode_ore,
            geode_obsidian,
        }
    })
    .collect();

    let answer_one = part_one(&blueprints);
    let answer_two = part_two(&blueprints[..3]);

    (answer_one, answer_two)
}

fn part_one(blueprints: &[Blueprint]) -> u32 {
    blueprints
    .iter()
    .enumerate()
    .fold(0, |total, (id, blueprint)| 
    total + (id as u32 + 1) * get_most_geodes(blueprint, 24))
}

fn part_two(blueprints: &[Blueprint]) -> u32 {
    blueprints
    .iter()
    .fold(1, |mult, blueprint| 
        mult * get_most_geodes(blueprint, 32))
}


fn get_most_geodes(blueprint: &Blueprint, end_time: u32) -> u32 {
    let mut most_geodes = 0;
    let mut strategies: Vec<(Robots, Materials, u32)> = vec![(
        Robots { ore: 1, clay: 0, obsidian: 0, geode: 0 },
        Materials { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        0,
    )];

    while !strategies.is_empty() {
        let (robots, materials, time) = strategies.pop().unwrap();

        let geode_at_end = materials.geode + (robots.geode * (end_time - time));
        if geode_at_end > most_geodes {
            most_geodes = materials.geode;
        }
        if time == end_time || geode_at_end + (((end_time - time + 1) * (end_time - time)) / 2) < most_geodes {
            continue;
        }

        for goal in 0..=3 {
            if goal == 0 && blueprint.get_ores().iter().all(|&req| req <= robots.ore) ||
            goal == 1 && robots.clay >= blueprint.obsidian_clay ||
            goal == 2 && robots.obsidian >= blueprint.geode_obsidian ||
            goal == 2 && robots.clay == 0 ||
            goal == 3 && robots.obsidian == 0 {
                continue;
            }

            let mut time = time;
            let mut robots = robots.clone();
            let mut materials = materials.clone();
            let mut done = false;
            while time < end_time && !done {
                if let Some((m, r)) = blueprint.buy(&materials, &robots, goal) {
                    materials = Materials {
                        ore: robots.ore + m.ore,
                        clay: robots.clay + m.clay,
                        obsidian: robots.obsidian + m.obsidian,
                        geode: robots.geode + m.geode,
                    };
                    robots = r;
                    done = true;
                }
                else {
                    robots.generate(&mut materials);
                }
                time += 1;
            }

            strategies.push((robots, materials, time));
        }
    }

    most_geodes
}
