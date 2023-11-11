pub fn run(input: String) -> (u32, u32) {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut solution = Solution {
        grid: input.to_owned(),
        flashed_coords: vec![],
        flash_count: 0,
    };

    let answer_one = solution.part_one();
    solution.grid = input;
    let answer_two = solution.part_two();

    (answer_one, answer_two)
}

struct Solution {
    grid: Vec<Vec<u32>>,
    flashed_coords: Vec<(usize, usize)>,
    flash_count: u32,
}

impl Solution {
    fn part_one(&mut self) -> u32 {
        for _ in 0..100 {
            self.flashed_coords = vec![];

            for y in 0..self.grid.len() {
                for x in 0..self.grid[0].len() {
                    self.grid[y][x] += 1;
                }
            }

            for y in 0..self.grid.len() {
                for x in 0..self.grid[0].len() {
                    if self.grid[y][x] > 9 {
                        self.adjacent(x, y);
                    }
                }
            }
        }

        return self.flash_count;
    }

    fn part_two(&mut self) -> u32 {
        let mut step = 0;
        while self.flashed_coords.len() != self.grid.len() * self.grid[0].len() {
            self.flashed_coords = vec![];
            step += 1;

            for y in 0..self.grid.len() {
                for x in 0..self.grid[0].len() {
                    self.grid[y][x] += 1;
                }
            }

            for y in 0..self.grid.len() {
                for x in 0..self.grid[0].len() {
                    if self.grid[y][x] > 9 {
                        self.adjacent(x, y);
                    }
                }
            }
        }

        return step;
    }

    fn adjacent(&mut self, x: usize, y: usize) {
        self.flashed_coords.push((x, y));
        self.grid[y][x] = 0;
        self.flash_count += 1;

        if x != 0 && !self.flashed_coords.contains(&(x - 1, y)) {
            self.grid[y][x - 1] += 1;
            if self.grid[y][x - 1] > 9 {
                self.adjacent(x - 1, y);
            }
        }

        if x != self.grid[0].len() - 1 && !self.flashed_coords.contains(&(x + 1, y)) {
            self.grid[y][x + 1] += 1;
            if self.grid[y][x + 1] > 9 {
                self.adjacent(x + 1, y);
            }
        }

        if y != 0 && !self.flashed_coords.contains(&(x, y - 1)) {
            self.grid[y - 1][x] += 1;
            if self.grid[y - 1][x] > 9 {
                self.adjacent(x, y - 1);
            }
        }

        if y != self.grid.len() - 1 && !self.flashed_coords.contains(&(x, y + 1)) {
            self.grid[y + 1][x] += 1;
            if self.grid[y + 1][x] > 9 {
                self.adjacent(x, y + 1);
            }
        }

        if x != 0 && y != 0 && !self.flashed_coords.contains(&(x - 1, y - 1)) {
            self.grid[y - 1][x - 1] += 1;
            if self.grid[y - 1][x - 1] > 9 {
                self.adjacent(x - 1, y - 1);
            }
        }

        if x != self.grid[0].len() - 1 && y != 0 && !self.flashed_coords.contains(&(x + 1, y - 1)) {
            self.grid[y - 1][x + 1] += 1;
            if self.grid[y - 1][x + 1] > 9 {
                self.adjacent(x + 1, y - 1);
            }
        }

        if x != self.grid[0].len() - 1
            && y != self.grid.len() - 1
            && !self.flashed_coords.contains(&(x + 1, y + 1))
        {
            self.grid[y + 1][x + 1] += 1;
            if self.grid[y + 1][x + 1] > 9 {
                self.adjacent(x + 1, y + 1);
            }
        }

        if x != 0 && y != self.grid.len() - 1 && !self.flashed_coords.contains(&(x - 1, y + 1)) {
            self.grid[y + 1][x - 1] += 1;
            if self.grid[y + 1][x - 1] > 9 {
                self.adjacent(x - 1, y + 1);
            }
        }
    }
}
