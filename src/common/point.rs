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
        self.magnitude().partial_cmp(&other.magnitude())
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub const UP: Point = Point { x: 0, y: 1 };
    pub const DOWN: Point = Point { x: 0, y: -1 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const LEFT: Point = Point { x: -1, y: 0 };

    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        let sum = (self.x.pow(2) + self.y.pow(2)) as f64;
        sum.sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
    
    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn angle_between(&self, other: &Self) -> f64 {
        (self.dot(other) as f64 / (self.magnitude() * other.magnitude())).acos()
    }

    pub fn angle(&self) -> f64 {
        self.angle_between(&Point::RIGHT)
    }

    pub fn rotate_clockwise(&mut self) {
        let x = self.x;
        let y = self.y;
        self.x = y;
        self.y = -x;   
    }

    pub fn rotate_anti_clockwise(&mut self) {
        let x = self.x;
        let y = self.y;
        self.x = -y;
        self.y = x;   
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        !self.is_zero() && !other.is_zero() && (
            (self.x * other.y) - (self.y * other.x)) == 0
    }

    pub fn is_zero(&self) -> bool {
        self == &Point::ORIGIN
    }
}

#[macro_export]
macro_rules! point {
    ($x:ident, $y:ident) => { point!(@ $x $y) };
    ($x:literal, $y:literal) => { point!(@ $x $y) };
    (@ $x:tt $y:tt) => { crate::common::Point::new($x, $y) };
}