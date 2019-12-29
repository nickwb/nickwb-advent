use num::integer::gcd;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ops::{Add, Div, Mul, Sub};

type Dimension = i32;

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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::xy(self.x - other.x, self.y - other.y)
    }
}

impl Mul<Dimension> for Point {
    type Output = Point;
    fn mul(self, rhs: Dimension) -> Point {
        Point::xy(self.x * rhs, self.y * rhs)
    }
}

impl Div<Dimension> for Point {
    type Output = Point;
    fn div(self, rhs: Dimension) -> Point {
        Point::xy(self.x / rhs, self.y / rhs)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
                    if self.side_length > self.map.height * 2
                        && self.side_length > self.map.width * 2
                    {
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
            if self.map.is_in_bounds(next) {
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

    fn is_asteroid_at(&self, point: Point) -> bool {
        self.asteroids.contains(&point)
    }

    fn is_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

fn get_observed_asteroids(map: &Map, point: Point) -> Vec<Point> {
    let mut observed: Vec<Point> = Vec::new();
    let mut blind_spots: HashSet<Point> = HashSet::new();

    for p in map.spiral(point) {
        if blind_spots.contains(&p) {
            continue;
        }
        if p == point {
            continue;
        }
        if map.is_asteroid_at(p) {
            observed.push(p);
            let mut step = p - point;
            let gcd = gcd(step.x, step.y);
            step = step / gcd;

            let mut multiple = 2;
            loop {
                let blind = (step * multiple) + point;
                if map.is_in_bounds(blind) {
                    blind_spots.insert(blind);
                    multiple += 1;
                    continue;
                }
                break;
            }
        }
    }

    observed
}

fn evaluate_point(map: &Map, point: Point) -> usize {
    get_observed_asteroids(map, point).len()
}

fn find_best_point(map: &Map) -> (Point, usize) {
    map.asteroids
        .par_iter()
        .map(|a| (*a, evaluate_point(map, *a)))
        .max_by_key(|t| t.1)
        .unwrap()
}

fn normalise_angle(theta: f64) -> f64 {
    let mut angle = theta;
    while angle < 0.0 {
        angle = angle + std::f64::consts::PI;
    }
    angle
}

fn get_destruction_angle(point: &Point, from: &Point) -> f64 {
    let relative = *from - *point;
    let x: f64 = relative.x.try_into().unwrap();
    let y: f64 = relative.y.try_into().unwrap();
    let theta = normalise_angle((y).atan2(x));
    let theta = normalise_angle(-std::f64::consts::FRAC_PI_2 - theta);
    theta
}

fn get_destruction_order(map: &mut Map, point: Point) -> Vec<Point> {
    // Start by removing ourselves from the map
    map.asteroids.remove(&point);
    let mut results: Vec<Point> = Vec::new();
    while !map.asteroids.is_empty() {
        let mut observable = get_observed_asteroids(map, point);
        for a in observable.iter() {
            map.asteroids.remove(a);
        }

        observable.sort_by(|a, b| {
            let a_angle = get_destruction_angle(a, &point);
            let b_angle = get_destruction_angle(b, &point);
            a_angle.partial_cmp(&b_angle).unwrap_or(Ordering::Equal)
        });

        results.append(&mut observable);
    }
    results
}

fn input() -> Map {
    let s = "
    ..#..###....#####....###........#
    .##.##...#.#.......#......##....#
    #..#..##.#..###...##....#......##
    ..####...#..##...####.#.......#.#
    ...#.#.....##...#.####.#.###.#..#
    #..#..##.#.#.####.#.###.#.##.....
    #.##...##.....##.#......#.....##.
    .#..##.##.#..#....#...#...#...##.
    .#..#.....###.#..##.###.##.......
    .##...#..#####.#.#......####.....
    ..##.#.#.#.###..#...#.#..##.#....
    .....#....#....##.####....#......
    .#..##.#.........#..#......###..#
    #.##....#.#..#.#....#.###...#....
    .##...##..#.#.#...###..#.#.#..###
    .#..##..##...##...#.#.#...#..#.#.
    .#..#..##.##...###.##.#......#...
    ...#.....###.....#....#..#....#..
    .#...###..#......#.##.#...#.####.
    ....#.##...##.#...#........#.#...
    ..#.##....#..#.......##.##.....#.
    .#.#....###.#.#.#.#.#............
    #....####.##....#..###.##.#.#..#.
    ......##....#.#.#...#...#..#.....
    ...#.#..####.##.#.........###..##
    .......#....#.##.......#.#.###...
    ...#..#.#.........#...###......#.
    .#.##.#.#.#.#........#.#.##..#...
    .......#.##.#...........#..#.#...
    .####....##..#..##.#.##.##..##...
    .#.#..###.#..#...#....#.###.#..#.
    ............#...#...#.......#.#..
    .........###.#.....#..##..#.##...";

    Map::from_string(s).unwrap()
}

fn part_one(map: &Map) -> (Point, usize) {
    find_best_point(map)
}

fn part_two(map: &mut Map) -> Point {
    let order = get_destruction_order(map, Point::xy(27, 19));
    *order.get(199).unwrap()
}

pub fn run_day_ten() {
    let mut map = input();
    let p1 = part_one(&map);
    println!("Day 10, Part 1: {} at ({}, {})", p1.1, p1.0.x, p1.0.y);
    let p2 = part_two(&mut map);
    println!("Day 10, Part 2: ({}, {})", p2.x, p2.y);
}

#[test]
fn example_1() {
    let s = "
    .#..#
    .....
    #####
    ....#
    ...##";

    let map = Map::from_string(s).unwrap();
    let best = find_best_point(&map);
    assert_eq!((Point::xy(3, 4), 8), best);
}

#[test]
fn example_2() {
    let s = "
    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####";

    let map = Map::from_string(s).unwrap();
    let best = find_best_point(&map);
    assert_eq!((Point::xy(5, 8), 33), best);
}

#[test]
fn example_3() {
    let s = "
    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.";

    let map = Map::from_string(s).unwrap();
    let best = find_best_point(&map);
    assert_eq!((Point::xy(1, 2), 35), best);
}

#[test]
fn example_4() {
    let s = "
    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..";

    let map = Map::from_string(s).unwrap();
    let best = find_best_point(&map);
    assert_eq!((Point::xy(6, 3), 41), best);
}

#[test]
fn example_5() {
    let s = "
    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";

    let mut map = Map::from_string(s).unwrap();
    let best = find_best_point(&map);
    assert_eq!((Point::xy(11, 13), 210), best);

    let order = get_destruction_order(&mut map, best.0);
    assert_eq!(Point::xy(11, 12), *order.get(0).unwrap());
    assert_eq!(Point::xy(12, 1), *order.get(1).unwrap());
    assert_eq!(Point::xy(12, 2), *order.get(2).unwrap());
    assert_eq!(Point::xy(12, 8), *order.get(9).unwrap());
    assert_eq!(Point::xy(16, 0), *order.get(19).unwrap());
    assert_eq!(Point::xy(16, 9), *order.get(49).unwrap());
    assert_eq!(Point::xy(10, 16), *order.get(99).unwrap());
    assert_eq!(Point::xy(9, 6), *order.get(198).unwrap());
    assert_eq!(Point::xy(8, 2), *order.get(199).unwrap());
    assert_eq!(Point::xy(11, 1), *order.get(298).unwrap());
}

#[test]
fn actual_part_1() {
    let map = input();
    let best = part_one(&map);
    assert_eq!((Point::xy(27, 19), 314), best);
}
