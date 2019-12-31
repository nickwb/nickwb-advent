use crate::point::Point;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub enum CoordinateMapping {
    YIncreasesDownwards,
    YIncreasesUpwards,
}

impl Direction {
    pub fn translate_point<T: Add<Output = T> + Sub<Output = T> + Copy>(
        &self,
        from: &Point<T>,
        distance: T,
        mapping: CoordinateMapping,
    ) -> Point<T> {
        match (self, mapping) {
            (Direction::Up, CoordinateMapping::YIncreasesUpwards) => {
                Point::xy(from.x, from.y + distance)
            }
            (Direction::Up, CoordinateMapping::YIncreasesDownwards) => {
                Point::xy(from.x, from.y - distance)
            }
            (Direction::Down, CoordinateMapping::YIncreasesUpwards) => {
                Point::xy(from.x, from.y - distance)
            }
            (Direction::Down, CoordinateMapping::YIncreasesDownwards) => {
                Point::xy(from.x, from.y + distance)
            }
            (Direction::Left, _) => Point::xy(from.x - distance, from.y),
            (Direction::Right, _) => Point::xy(from.x + distance, from.y),
        }
    }

    pub fn try_parse_char(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn get_orientation(&self) -> Orientation {
        match self {
            Direction::Up => Orientation::Vertical,
            Direction::Down => Orientation::Vertical,
            Direction::Left => Orientation::Horizontal,
            Direction::Right => Orientation::Horizontal,
        }
    }

    pub fn turned_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
