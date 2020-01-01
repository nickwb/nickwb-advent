use crate::util::{self, CoordinateMapping, Direction, Orientation};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;

type Grid = i64;
type Point = crate::util::Point<Grid>;

struct Intersection {
    point: Point,
    total_length: Grid,
}

type Intersections = Vec<Intersection>;

#[derive(Debug, PartialEq)]
struct Segment {
    start: Point,
    end: Point,
    length: Grid,
    dir: Direction,
    prev_length: Grid,
}

lazy_static! {
    static ref SEGMENT_PATTERN: Regex = Regex::new(r"([LRUD])(\d+)").unwrap();
}

impl Segment {
    fn new(spec: &str, start: Point, prev_length: Grid) -> Option<Segment> {
        let captures = SEGMENT_PATTERN.captures(spec)?;
        let dir = Direction::try_parse_char(captures.get(1)?.as_str().chars().next()?)?;
        let length = captures.get(2)?.as_str().parse::<Grid>().ok()?;
        let end = dir.translate_point(&start, length, CoordinateMapping::YIncreasesUpwards);
        Some(Segment {
            start,
            end,
            dir,
            length,
            prev_length,
        })
    }

    fn left_most(&self) -> Grid {
        std::cmp::min(self.start.x, self.end.x)
    }

    fn right_most(&self) -> Grid {
        std::cmp::max(self.start.x, self.end.x)
    }

    fn top_most(&self) -> Grid {
        std::cmp::max(self.start.y, self.end.y)
    }

    fn bottom_most(&self) -> Grid {
        std::cmp::min(self.start.y, self.end.y)
    }

    fn add_intersections(&self, other: &Segment, list: &mut Intersections) {
        // First decide if it's even possible for these two segments to intersect
        // Then, go ahead and measure the intersection
        match (self.dir.get_orientation(), other.dir.get_orientation()) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                if self.left_most() <= other.start.x
                    && self.right_most() >= other.start.x
                    && other.bottom_most() <= self.start.y
                    && other.top_most() >= self.start.y
                {
                    Segment::measure_intersections(self, other, list);
                }
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                if other.left_most() <= self.start.x
                    && other.right_most() >= self.start.x
                    && self.bottom_most() <= other.start.y
                    && self.top_most() >= other.start.y
                {
                    Segment::measure_intersections(self, other, list);
                }
            }
            (Orientation::Horizontal, Orientation::Horizontal) => {
                if self.start.y != other.start.y
                    || self.right_most() < other.left_most()
                    || other.right_most() < self.left_most()
                {
                    return;
                }
                Segment::measure_intersections(self, other, list);
            }
            (Orientation::Vertical, Orientation::Vertical) => {
                if self.start.x != other.start.x
                    || self.bottom_most() < other.top_most()
                    || other.bottom_most() < self.top_most()
                {
                    return;
                }
                Segment::measure_intersections(self, other, list);
            }
        }
    }

    fn measure_intersections(a: &Segment, b: &Segment, list: &mut Intersections) {
        // Decide which is the shortest segment, and which is the longest
        let (shortest, longest) = if a.length > b.length { (b, a) } else { (a, b) };
        let mut maybe_intersections: HashMap<Point, Grid> =
            HashMap::with_capacity(shortest.length.try_into().unwrap());
        // Walk the length of the shortest segment
        {
            let mut location = shortest.start;
            let mut walk_length = 0;
            while location != shortest.end {
                // If we intersect at this point, the path along this segment will be this long
                let path_length = shortest.prev_length + walk_length;
                maybe_intersections.insert(location, path_length);
                // Keep walking
                location = shortest.dir.translate_point(
                    &location,
                    1,
                    CoordinateMapping::YIncreasesUpwards,
                );
                walk_length += 1;
            }
        }
        // Now find which parts of the longer segment intersect
        {
            let mut location = longest.start;
            let mut walk_length = 0;
            while location != longest.end {
                let intersection = maybe_intersections.get(&location);
                // Found an intersection at this location
                if let Some(intersected_path_length) = intersection {
                    let path_length = longest.prev_length + walk_length;
                    let total_length = intersected_path_length + path_length;
                    list.push(Intersection {
                        point: location,
                        total_length: total_length,
                    });
                }
                // Keep walking
                location =
                    longest
                        .dir
                        .translate_point(&location, 1, CoordinateMapping::YIncreasesUpwards);
                walk_length += 1;
            }
        }
    }
}

struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn new(spec: &str) -> Option<Wire> {
        Some(Wire {
            segments: spec.split(',').fold(
                Some(Vec::new()),
                |segments: Option<Vec<Segment>>, s| {
                    let mut segments = segments?;
                    let (start, prev_length) = match segments.last() {
                        None => (Point { x: 0, y: 0 }, 0),
                        Some(s) => (s.end, s.prev_length + s.length),
                    };
                    segments.push(Segment::new(s, start, prev_length)?);
                    Some(segments)
                },
            )?,
        })
    }

    fn add_intersections(&self, other: &Wire, list: &mut Intersections) {
        for a_seg in self.segments.iter() {
            for b_seg in other.segments.iter() {
                a_seg.add_intersections(b_seg, list);
            }
        }
    }
}

struct WireSet {
    wires: Vec<Wire>,
}

impl WireSet {
    fn from<'a, I: Iterator<Item = &'a str>>(lines: I) -> Option<WireSet> {
        Some(WireSet {
            wires: lines.fold(Some(Vec::new()), |wires: Option<Vec<Wire>>, w| {
                let mut wires = wires?;
                wires.push(Wire::new(w)?);
                Some(wires)
            })?,
        })
    }

    fn all_intersections(&self) -> Intersections {
        let mut results: Intersections = Vec::new();
        for i in 0..self.wires.len() {
            for j in i + 1..self.wires.len() {
                self.wires[i].add_intersections(&self.wires[j], &mut results);
            }
        }
        results
    }
}

fn lowest_manhattan_excluding_origin(points: &Intersections) -> Option<Grid> {
    points
        .iter()
        .filter(|p| !(p.point.x == 0 && p.point.y == 0))
        .map(|p| p.point.x.abs() + p.point.y.abs())
        .min()
}

fn lowest_steps_excluding_origin(points: &Intersections) -> Option<Grid> {
    points
        .iter()
        .filter(|p| !(p.point.x == 0 && p.point.y == 0))
        .map(|p| p.total_length)
        .min()
}

fn input() -> WireSet {
    WireSet::from(util::read_file("inputs/day3.txt").lines()).unwrap()
}

fn calculate_day_three() -> (Grid, Grid) {
    let set = input();
    let intersections = set.all_intersections();
    let manhattan = lowest_manhattan_excluding_origin(&intersections).unwrap();
    let steps = lowest_steps_excluding_origin(&intersections).unwrap();
    (manhattan, steps)
}

pub fn run_day_three() {
    let (manhattan, steps) = calculate_day_three();
    println!("Day 3, Part 1: {}", manhattan);
    println!("Day 3, Part 2: {}", steps);
}

#[test]
fn parse_test() {
    let wire = Wire::new("R8,U5,L5,D3").unwrap();
    assert_eq!(4, wire.segments.len());

    let s1 = &wire.segments[0];
    assert_eq!(
        &Segment {
            start: Point::xy(0, 0),
            end: Point::xy(8, 0),
            dir: Direction::Right,
            length: 8,
            prev_length: 0,
        },
        s1
    );

    let s2 = &wire.segments[1];
    assert_eq!(
        &Segment {
            start: Point::xy(8, 0),
            end: Point::xy(8, 5),
            dir: Direction::Up,
            length: 5,
            prev_length: 8,
        },
        s2
    );

    let s3 = &wire.segments[2];
    assert_eq!(
        &Segment {
            start: Point::xy(8, 5),
            end: Point::xy(3, 5),
            dir: Direction::Left,
            length: 5,
            prev_length: 13,
        },
        s3
    );

    let s4 = &wire.segments[3];
    assert_eq!(
        &Segment {
            start: Point::xy(3, 5),
            end: Point::xy(3, 2),
            dir: Direction::Down,
            length: 3,
            prev_length: 18,
        },
        s4
    );
}

#[test]
fn example_distance_1() {
    let lines = ["R8,U5,L5,D3", "U7,R6,D4,L4"];

    let set = WireSet::from(lines.iter().map(|x| *x)).unwrap();
    let intersections = set.all_intersections();
    let manhattan = lowest_manhattan_excluding_origin(&intersections).unwrap();
    assert_eq!(6, manhattan);

    let steps = lowest_steps_excluding_origin(&intersections).unwrap();
    assert_eq!(30, steps);
}

#[test]
fn example_distance_2() {
    let lines = [
        "R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83",
    ];

    let set = WireSet::from(lines.iter().map(|x| *x)).unwrap();
    let intersections = set.all_intersections();

    let manhattan = lowest_manhattan_excluding_origin(&intersections).unwrap();
    assert_eq!(159, manhattan);

    let steps = lowest_steps_excluding_origin(&intersections).unwrap();
    assert_eq!(610, steps);
}

#[test]
fn example_distance_3() {
    let lines = [
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    ];

    let set = WireSet::from(lines.iter().map(|x| *x)).unwrap();
    let intersections = set.all_intersections();
    let manhattan = lowest_manhattan_excluding_origin(&intersections).unwrap();
    assert_eq!(135, manhattan);

    let steps = lowest_steps_excluding_origin(&intersections).unwrap();
    assert_eq!(410, steps);
}

#[test]
fn actual_day_3() {
    let (manhattan, steps) = calculate_day_three();
    assert_eq!(352, manhattan);
    assert_eq!(43848, steps);
}
