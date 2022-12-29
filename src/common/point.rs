use std::{fmt::Display, ops::{Sub, Add, Mul, Div}};


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

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn modulus(&self) -> i64 {
        let sum = (self.x.pow(2) + self.y.pow(2)) as f64;
        sum.sqrt() as i64
    }

    pub fn normalize(&mut self) -> &Self {
        let magnitude = self.modulus();
        self.x /= magnitude;
        self.y /= magnitude;
        self 
    }

    pub fn dot(&self, rhs: &Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
}