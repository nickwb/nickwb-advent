use crate::intcode::*;
use std::collections::HashMap;

type Point = crate::util::Point<MemoryCell>;

#[derive(PartialEq)]
enum CellType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl CellType {
    fn from_numeric(value: MemoryCell) -> CellType {
        match value {
            0 => CellType::Empty,
            1 => CellType::Wall,
            2 => CellType::Block,
            3 => CellType::Paddle,
            4 => CellType::Ball,
            _ => panic!("Invalid cell type: {}", value),
        }
    }
}

struct ScreenBuffer {
    cells: HashMap<Point, CellType>,
    state: SinkState,
}

impl ScreenBuffer {
    fn new() -> ScreenBuffer {
        ScreenBuffer {
            cells: HashMap::new(),
            state: SinkState::ZeroReceived,
        }
    }
}

enum SinkState {
    ZeroReceived,
    OneReceived(MemoryCell),
    TwoReceived(MemoryCell, MemoryCell),
}

impl OutputSink for ScreenBuffer {
    fn write(&mut self, value: MemoryCell) {
        let new_state = match self.state {
            SinkState::ZeroReceived => SinkState::OneReceived(value),
            SinkState::OneReceived(x) => SinkState::TwoReceived(x, value),
            SinkState::TwoReceived(x, y) => {
                let point = Point::xy(x, y);
                let cell_type = CellType::from_numeric(value);
                self.cells.insert(point, cell_type);
                SinkState::ZeroReceived
            }
        };
        self.state = new_state;
    }
}

fn inputs() -> Vec<MemoryCell> {
    crate::util::read_int_array("inputs/day13.txt")
}

fn calculate_part_one() -> usize {
    let input = NoInput {};
    let output = ScreenBuffer::new();
    let mut computer = Computer::new(inputs(), input, output);
    computer.enable_extra_memory();
    computer.run_until_halt().unwrap();

    computer
        .output()
        .cells
        .values()
        .filter(|&c| c == &CellType::Block)
        .count()
}

pub fn run_day_thirteen() {
    println!("Day 13, Part 1: {}", calculate_part_one());
}

#[test]
fn actual_part_1() {
    let result = calculate_part_one();
    assert_eq!(341, result);
}
