use std::collections::HashMap;

use crate::intcode::*;

pub fn run_day_fifteen() {
    println!("Day 15: TODO");
}

fn inputs() -> Vec<MemoryCell> {
    crate::util::read_int_array("inputs/day13.txt")
}

struct DroidInput {
    next_input: Option<MemoryCell>,
}

impl InputSource for DroidInput {
    fn next(&mut self) -> Option<MemoryCell> {
        self.next_input.take()
    }
}

struct DroidOutput {
    last_output: Option<MemoryCell>,
}

impl OutputSink for DroidOutput {
    fn write(&mut self, value: MemoryCell) {
        match self.last_output {
            Some(_) => panic!("Output buffer is full!"),
            None => self.last_output = Some(value),
        }
    }
}

type DroidComputer = Computer<Vec<MemoryCell>, DroidInput, DroidOutput>;

type Point = crate::util::Point<MemoryCell>;

const ZERO_POINT: Point = Point::xy(0, 0);

struct Orchestrator {
    computer: DroidComputer,
    location: Point,
    oxygen: Option<Point>,
    observations: HashMap<Point, Observation>,
}

enum Observation {
    Empty,
    Wall,
    OxygenSystem,
}

enum Direction {
    North,
    South,
    East,
    West,
}

enum MoveOutcome {
    ArrivedAtDestination,
    MoveSuccessful,
    StruckWall,
}

impl Orchestrator {
    fn new(state: Vec<MemoryCell>) -> Orchestrator {
        let input = DroidInput { next_input: None };
        let output = DroidOutput { last_output: None };
        let computer = DroidComputer::new(state, input, output);
        Orchestrator {
            computer,
            location: ZERO_POINT,
            oxygen: None,
            observations: HashMap::new(),
        }
    }

    fn move_droid(&mut self, direction: Direction) -> MoveOutcome {
        unimplemented!();
    }
}
