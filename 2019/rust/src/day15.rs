use slotmap::new_key_type;

use crate::intcode::*;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

pub fn run_day_fifteen() {
    println!("Day 15: TODO");
}

fn inputs() -> Vec<MemoryCell> {
    crate::util::read_int_array("inputs/day15.txt")
}

type DroidComputer = Computer<Vec<MemoryCell>, Rc<RefCell<DroidIo>>, Rc<RefCell<DroidIo>>>;

struct Droid {
    computer: DroidComputer,
    map: HashMap<Point, Observation>,
    location: Point,
    path_to: HashMap<Point, PathNode>,
}

struct PathNode {
    direction: Direction,
    from: Point,
}

impl Droid {
    fn new(program: Vec<MemoryCell>) -> Droid {
        let io = Rc::new(RefCell::new(DroidIo {
            next_input: None,
            next_output: None,
        }));

        let mut computer: DroidComputer = Computer::new(program, io.clone(), io);
        computer.enable_extra_memory();

        Self {
            computer,
            map: HashMap::new(),
            location: ZERO_POINT,
            path_to: HashMap::new(),
        }
    }

    fn find_oxygen_bfs(&mut self) -> Option<Point> {
        // Mark 0,0 as explored
        self.map.insert(ZERO_POINT, Observation::Empty);

        let mut candidates: VecDeque<Point> = VecDeque::new();

        // Initialise candidates with the neighbours of 0,0

        let mut path: VecDeque<Direction> = VecDeque::new();
        while let Some(next) = candidates.pop_back() {
            // Determine the path to the candidate
            path.clear();
            self.build_path_to(&next, &mut path);

            // Start at 0,0
            // Follow the path to the candidate
            let mut observation = Observation::Empty;
            for &d in path.iter() {
                observation = self.traverse(d);
            }

            // Check for oxygen
            if observation == Observation::OxygenSystem {
                return Some(self.location);
            }

            // Update the map
            self.map.insert(self.location, observation);

            // Register any new candidates
            // Return to 0,0
            self.backtrack(&path);
        }
        todo!()
    }

    fn backtrack(&mut self, path: &VecDeque<Direction>) {
        for d in path {
            let _ = match d {
                Direction::North => self.traverse(Direction::South),
                Direction::South => self.traverse(Direction::North),
                Direction::East => self.traverse(Direction::West),
                Direction::West => self.traverse(Direction::East),
            };
        }
    }

    fn traverse(&mut self, direction: Direction) -> Observation {
        let input = match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        };

        {
            let mut io = self.computer.input().borrow_mut();
            io.next_input = Some(input);
        }

        let result = self.computer.resume();
        match result {
            Ok(StepResult::WaitingOnInput) => (),
            _ => panic!("Computer didn't traverse as expected"),
        }

        let observation = {
            let mut io = self.computer.output().borrow_mut();
            io.next_output.take()
        };

        let observation = match observation {
            Some(0) => Observation::Wall,
            Some(1) => Observation::Empty,
            Some(2) => Observation::OxygenSystem,
            Some(_) => panic!("Unexpected output from computer"),
            None => panic!("Computer didn't produce output"),
        };

        if observation != Observation::Wall {
            let movement = match direction {
                Direction::North => Point::xy(0, 1),
                Direction::South => Point::xy(0, -1),
                Direction::East => Point::xy(1, 0),
                Direction::West => Point::xy(-1, 0),
            };
            self.location = self.location + movement;
        }

        observation
    }

    fn build_path_to(&self, destination: &Point, buffer: &mut VecDeque<Direction>) -> Option<()> {
        let mut location = destination;
        while location.x != 0 && location.y != 0 {
            let path = self.path_to.get(location)?;
            buffer.push_front(path.direction);
            location = &path.from;
        }
        Some(())
    }
}

struct DroidIo {
    next_input: Option<MemoryCell>,
    next_output: Option<MemoryCell>,
}

impl InputSource for Rc<RefCell<DroidIo>> {
    fn next(&mut self) -> Option<MemoryCell> {
        let mut io = self.borrow_mut();
        io.next_input.take()
    }
}

impl OutputSink for Rc<RefCell<DroidIo>> {
    fn write(&mut self, value: MemoryCell) {
        let mut io = self.borrow_mut();
        io.next_output = Some(value);
    }
}

type Point = crate::util::Point<MemoryCell>;

const ZERO_POINT: Point = Point::xy(0, 0);

#[derive(Clone, Copy, PartialEq)]
enum Observation {
    Empty,
    Wall,
    OxygenSystem,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq)]
enum MoveOutcome {
    ArrivedAtDestination,
    MoveSuccessful,
    StruckWall,
}
