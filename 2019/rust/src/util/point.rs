use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn xy(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point::xy(self.x + other.x, self.y + other.y)
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point::xy(self.x - other.x, self.y - other.y)
    }
}

impl<T: Copy + Mul<T, Output = T>> Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Point<T> {
        Point::xy(self.x * rhs, self.y * rhs)
    }
}

impl<T: Copy + Div<T, Output = T>> Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Point<T> {
        Point::xy(self.x / rhs, self.y / rhs)
    }
}
