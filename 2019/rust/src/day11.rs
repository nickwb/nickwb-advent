use crate::direction::{CoordinateMapping, Direction};
use crate::intcode::*;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;

type Coordinate = i32;
type Point = crate::point::Point<Coordinate>;

#[derive(Clone, Copy)]
enum Colour {
    Black,
    White,
}

struct PaintingRobot {
    colours: HashMap<Point, Colour>,
    position: Point,
    direction: Direction,
}

impl PaintingRobot {
    fn new(start_color: Colour) -> PaintingRobot {
        let mut robot = PaintingRobot {
            colours: HashMap::new(),
            position: Point::xy(0, 0),
            direction: Direction::Up,
        };
        robot.colours.insert(robot.position, start_color);
        robot
    }

    fn get_color_at_point(&self, point: &Point) -> Colour {
        self.colours.get(point).copied().unwrap_or(Colour::Black)
    }

    fn apply_paint(&mut self, colour: MemoryCell) {
        let paint_colour = match colour {
            0 => Colour::Black,
            1 => Colour::White,
            _ => {
                panic!("Unknown paint colour");
            }
        };
        self.colours.insert(self.position, paint_colour);
    }

    fn turn_and_advance(&mut self, direction: MemoryCell) {
        self.direction = match direction {
            0 => self.direction.turned_left(),
            1 => self.direction.turned_right(),
            _ => {
                panic!("Unknown direction");
            }
        };
        self.position =
            self.direction
                .translate_point(&self.position, 1, CoordinateMapping::YIncreasesUpwards);
    }
}

struct RobotInput<'a> {
    robot: &'a RefCell<PaintingRobot>,
}

impl<'a> InputSource for RobotInput<'a> {
    fn next(&mut self) -> Option<MemoryCell> {
        let robot = self.robot.borrow();
        match robot.get_color_at_point(&robot.position) {
            Colour::Black => Some(0),
            Colour::White => Some(1),
        }
    }
}

struct RobotOutput<'a> {
    robot: &'a RefCell<PaintingRobot>,
    instruction: usize,
}

impl<'a> OutputSink for RobotOutput<'a> {
    fn write(&mut self, value: MemoryCell) {
        let mut robot = self.robot.borrow_mut();
        if self.instruction % 2 == 0 {
            robot.apply_paint(value);
        } else {
            robot.turn_and_advance(value);
        }
        self.instruction += 1;
    }
}

fn inputs() -> Vec<MemoryCell> {
    std::fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<MemoryCell>().unwrap())
        .collect()
}

fn run_robot_until_completion(start_color: Colour) -> PaintingRobot {
    let cell = RefCell::new(PaintingRobot::new(start_color));
    let input = RobotInput { robot: &cell };
    let output = RobotOutput {
        robot: &cell,
        instruction: 0,
    };
    let mut computer = Computer::new(inputs(), input, output);
    computer.enable_extra_memory();
    computer.run_until_halt().unwrap();

    cell.into_inner()
}

fn calculate_part_1() -> usize {
    let robot = run_robot_until_completion(Colour::Black);
    robot.colours.len()
}

fn find_bounds(robot: &PaintingRobot) -> (Coordinate, Coordinate, Coordinate, Coordinate) {
    robot
        .colours
        .keys()
        .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), point| {
            (
                min(min_x, point.x),
                max(max_x, point.x),
                min(min_y, point.y),
                max(max_y, point.y),
            )
        })
}

fn render_part_2() {
    let robot = run_robot_until_completion(Colour::White);
    let (min_x, max_x, min_y, max_y) = find_bounds(&robot);
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            match robot.get_color_at_point(&Point::xy(j, i)) {
                Colour::Black => {
                    print!(" ");
                    break;
                }
                Colour::White => {
                    print!("█");
                    break;
                }
            }
        }
        println!("");
    }
}

pub fn run_day_eleven() {
    println!("Day 8, Part 1: {}", calculate_part_1());
    render_part_2();
}

#[test]
fn actual_day_11() {
    assert_eq!(2339, calculate_part_1());
}
