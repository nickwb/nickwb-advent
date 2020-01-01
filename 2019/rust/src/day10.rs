use crate::util::{self, CoordinateMapping, Direction};
use num::integer::gcd;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;

type Dimension = i32;
type Point = crate::util::Point<Dimension>;

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
    direction: Direction,
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
            direction: Direction::Down,
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
                self.direction = self.direction.turned_right();
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
            self.position = self.direction.translate_point(
                &self.position,
                1,
                CoordinateMapping::YIncreasesDownwards,
            );
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

const TWO_PI: f64 = (2.0 * std::f64::consts::PI);
const PI_ON_TWO: f64 = std::f64::consts::FRAC_PI_2;

fn normalise_angle(theta: f64) -> f64 {
    let mut angle = theta;
    while angle < 0.0 {
        angle = angle + TWO_PI;
    }
    while angle > TWO_PI {
        angle = angle - TWO_PI;
    }
    if angle.abs() <= std::f64::EPSILON || (angle - TWO_PI).abs() <= std::f64::EPSILON {
        angle = 0.0;
    }
    angle
}

fn get_destruction_angle(point: &Point, from: &Point) -> f64 {
    let delta = *point - *from;
    let x: f64 = delta.x.try_into().unwrap();
    let y: f64 = delta.y.try_into().unwrap();
    let theta = (-y).atan2(x);
    let theta = normalise_angle((TWO_PI - theta) + PI_ON_TWO);
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
    let text = util::read_file("inputs/day10.txt");
    Map::from_string(&text).unwrap()
}

fn part_one(map: &Map) -> (Point, usize) {
    find_best_point(map)
}

fn part_two(map: &mut Map, best_point: Point) -> Point {
    let order = get_destruction_order(map, best_point);
    order[199]
}

fn calculate_day_ten() -> (Point, usize, Point) {
    let mut map = input();
    let (best, observed) = part_one(&map);
    let two_hundred = part_two(&mut map, best);
    (best, observed, two_hundred)
}

pub fn run_day_ten() {
    let (best, observed, two_hundred) = calculate_day_ten();
    println!("Day 10, Part 1: {} at ({}, {})", observed, best.x, best.y);
    println!("Day 10, Part 2: ({}, {})", two_hundred.x, two_hundred.y);
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
    assert_eq!(Point::xy(11, 12), order[0]);
    assert_eq!(Point::xy(12, 1), order[1]);
    assert_eq!(Point::xy(12, 2), order[2]);
    assert_eq!(Point::xy(12, 8), order[9]);
    assert_eq!(Point::xy(16, 0), order[19]);
    assert_eq!(Point::xy(16, 9), order[49]);
    assert_eq!(Point::xy(10, 16), order[99]);
    assert_eq!(Point::xy(9, 6), order[198]);
    assert_eq!(Point::xy(8, 2), order[199]);
    assert_eq!(Point::xy(11, 1), order[298]);
}

#[test]
fn actual_part_1_and_2() {
    let (best, observed, two_hundred) = calculate_day_ten();
    assert_eq!(Point::xy(27, 19), best);
    assert_eq!(314, observed);
    assert_eq!(Point::xy(15, 13), two_hundred);
}
