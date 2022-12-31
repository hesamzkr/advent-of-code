use std::{fmt::Display, ops::{Sub, Add, Mul, Div}, cmp::Ordering};


#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i64> for Point {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.modulus().partial_cmp(&other.modulus())
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Point {
    pub const UP: Point = Point { x: 0, y: 1 };
    pub const DOWN: Point = Point { x: 0, y: -1 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const LEFT: Point = Point { x: -1, y: 0 };

    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn modulus(&self) -> f64 {
        let sum = (self.x.pow(2) + self.y.pow(2)) as f64;
        sum.sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
    
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot(other) / (self.modulus() * other.modulus())).acos()
    }

    pub fn angle(&self) -> f64 {
        todo!()
    }

    pub fn rotate(&mut self, degrees: i64) {
        let radians = degrees as f64 * PI / 180.0;
        let x_degrees = self.x as f64 * radians;
        let y_degrees = self.y as f64 * radians;
        self.x = (x_degrees.cos() - y_degrees.sin()) as i64;
        self.y = (x_degrees.sin() + y_degrees.cos()) as i64;
    }

    pub fn perpendicular(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        self.is_vector() && other.is_vector() && ( 
        self.x == 0 && other.x == 0 ||
        self.y == 0 && other.y == 0 ||
        self.x.checked_div(other.x).unwrap_or(0) == self.y.checked_div(other.y).unwrap_or(0)) 
    }

    pub fn is_vector(&self) -> bool {
        self.x != 0 || self.y != 0
    }
}