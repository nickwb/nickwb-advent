use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::ops::Add;

type Dimension = isize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: Dimension,
    y: Dimension,
}

impl Point {
    fn xy(x: Dimension, y: Dimension) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::xy(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    width: Dimension,
    height: Dimension,
    asteroids: HashSet<Point>,
}

struct SpiralIter<'a> {
    map: &'a Map,
    position: Point,
    side_length: Dimension,
    direction: Point,
    step: Dimension,
    turns: usize,
    complete: bool,
}

impl<'a> SpiralIter<'a> {
    fn begin(map: &'a Map, start: Point) -> SpiralIter<'a> {
        SpiralIter {
            map,
            position: Point::xy(start.x, start.y),
            side_length: 1,
            direction: Point::xy(0, 1), // Downwards
            step: 0,
            turns: 0,
            complete: false,
        }
    }
}

impl<'a> Iterator for SpiralIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        loop {
            if self.complete {
                return None;
            }
            // Are we ready to turn?
            if self.step == self.side_length {
                // Rotate right
                self.direction = Point::xy(-1 * self.direction.y, self.direction.x);
                self.turns += 1;
                self.step = 0;
                // Every second turn, the length gets 1 unit longer
                if self.turns % 2 == 0 {
                    self.side_length += 1;

                    // The spiral is now bigger than the map
                    if self.side_length > self.map.height && self.side_length > self.map.width {
                        self.complete = true;
                    }
                }
            }
            // Item to return is wherever we were at the start of this iteration
            let next = self.position;
            // Now update to the new position
            self.position = self.position + self.direction;
            self.step += 1;

            // Return this position from the iterator if its within bounds
            if next.x >= 0 && next.x < self.map.width && next.y >= 0 && next.y < self.map.height {
                return Some(next);
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
                        Some(Point::xy(i.try_into().ok()?, y))
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

    fn spiral(&self, start: Point) -> SpiralIter {
        SpiralIter::begin(self, start)
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
        Point::xy(1, 0),
        Point::xy(4, 0),
        Point::xy(0, 2),
        Point::xy(1, 2),
        Point::xy(2, 2),
        Point::xy(3, 2),
        Point::xy(4, 2),
        Point::xy(4, 3),
        Point::xy(3, 4),
        Point::xy(4, 4),
    ];

    let map = Map::from_string(s).unwrap();

    assert_eq!(
        Map {
            width: 5,
            height: 5,
            asteroids: HashSet::from_iter(asteroids.iter().copied())
        },
        map
    );

    let i = map.spiral(Point::xy(2, 2));
    for p in i {
        println!("At {:?}", p);
    }
}
