use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;

type Dimension = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point(Dimension, Dimension);

impl Point {
    fn x(&self) -> Dimension {
        self.0
    }
    fn y(&self) -> Dimension {
        self.1
    }
}

enum Occupancy {
    Empty,
    Asteroid,
}

struct Space {
    point: Point,
    occupancy: Occupancy,
}

#[derive(Debug, PartialEq)]
struct Map {
    width: Dimension,
    height: Dimension,
    asteroids: HashSet<Point>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    fn traverse(&self, point: Point) -> Point {
        match self {
            Direction::Down => Point(point.x(), point.y() + 1),
            Direction::Left => Point(point.x() - 1, point.y()),
            Direction::Up => Point(point.x(), point.y() - 1),
            Direction::Right => Point(point.x() + 1, point.y()),
        }
    }
}

struct SpiralMovement {
    next: Point,
    min: Point,
    max: Point,
    direction: Direction,
}

struct SpiralIter<'a> {
    map: &'a Map,
    start: Point,
    movement: Option<SpiralMovement>,
}

impl<'a> SpiralIter<'a> {
    fn begin(map: &'a Map, start: Point) -> SpiralIter<'a> {
        SpiralIter {
            map,
            start,
            movement: None,
        }
    }
}

impl<'a> Iterator for SpiralIter<'a> {
    type Item = Space;

    fn next(&mut self) -> Option<Space> {
        loop {
            if let None = self.movement {
                let dir = Direction::Down;
                self.movement = Some(SpiralMovement {
                    next: dir.traverse(self.start),
                    min: Point(0, 0),
                    max: Point(0, 0),
                    direction: dir,
                });
                return Some(Space {
                    point: self.start,
                    occupancy: Occupancy::Empty,
                });
            }
        }
    }
}

impl Map {
    fn from_string(map_string: &str) -> Option<Map> {
        let rows = map_string.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
        let map = rows.fold(
            Some(Map {
                width: 0,
                height: 0,
                asteroids: HashSet::new(),
            }),
            |m, line| {
                let mut map = m?;
                let y = map.height;
                let line_width = line.len().try_into().ok()?;
                if map.width == 0 {
                    map.width = line_width;
                } else if map.width != line_width {
                    panic!("Invalid map");
                }
                let asteroids = line.chars().enumerate().filter_map(|(i, c)| {
                    if c == '#' {
                        Some(Point(i.try_into().ok()?, y))
                    } else {
                        None
                    }
                });
                for a in asteroids {
                    map.asteroids.insert(a);
                }
                map.height += 1;
                Some(map)
            },
        );
        map
    }
}

#[test]
fn example_1() {
    let s = "
.#..#
.....
#####
....#
...##";

    let asteroids = [
        Point(1, 0),
        Point(4, 0),
        Point(0, 2),
        Point(1, 2),
        Point(2, 2),
        Point(3, 2),
        Point(4, 2),
        Point(4, 3),
        Point(3, 4),
        Point(4, 4),
    ];

    assert_eq!(
        Map {
            width: 5,
            height: 5,
            asteroids: HashSet::from_iter(asteroids.iter().copied())
        },
        Map::from_string(s).unwrap()
    );
}
