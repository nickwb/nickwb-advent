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
    last_score: MemoryCell,
}

impl ScreenBuffer {
    fn new() -> ScreenBuffer {
        ScreenBuffer {
            cells: HashMap::new(),
            state: SinkState::ZeroReceived,
            last_score: 0,
        }
    }

    fn render_frame(&self) -> (Option<Point>, Option<Point>) {
        let (min_x, max_x, min_y, max_y) =
            self.cells.keys().fold((0, 0, 0, 0), |extremes, point| {
                (
                    extremes.0.min(point.x),
                    extremes.1.max(point.x),
                    extremes.2.min(point.y),
                    extremes.3.max(point.y),
                )
            });

        let mut paddle_at: Option<Point> = None;
        let mut ball_at: Option<Point> = None;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let char = match self.cells.get(&Point::xy(x, y)) {
                    Some(CellType::Ball) => {
                        ball_at = Some(Point::xy(x, y));
                        'O'
                    }
                    Some(CellType::Block) => 'X',
                    Some(CellType::Wall) => '|',
                    Some(CellType::Paddle) => {
                        paddle_at = Some(Point::xy(x, y));
                        '^'
                    }
                    _ => ' ',
                };
                print!("{}", char);
            }
            println!("");
        }

        println!("Score: {}", self.last_score);
        (paddle_at, ball_at)
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
                if x == -1 && y == 0 {
                    self.last_score = value;
                } else {
                    let point = Point::xy(x, y);
                    let cell_type = CellType::from_numeric(value);
                    self.cells.insert(point, cell_type);
                }
                SinkState::ZeroReceived
            }
        };
        self.state = new_state;
    }
}

pub struct GameState {
    comp: Computer<Vec<MemoryCell>, BufferInput, ScreenBuffer>,
    paddle_at: Point,
    ball_at: Point,
}

impl GameState {
    pub fn new() -> GameState {
        let input = BufferInput::new(1);
        let output = ScreenBuffer::new();
        let mut state = inputs();
        state[0] = 2;
        let mut computer = Computer::new(state, input, output);
        computer.enable_extra_memory();

        GameState {
            comp: computer,
            paddle_at: Point::xy(0, 0),
            ball_at: Point::xy(0, 0),
        }
    }

    pub fn run_one_cycle(&mut self) -> bool {
        let game_over = match self.comp.resume() {
            Ok(StepResult::WaitingOnInput) => false,
            Ok(StepResult::Halt) => true,
            _ => panic!("The computer got in to an invalid state"),
        };

        let (paddle_at, ball_at) = self.comp.output().render_frame();

        if let Some(paddle) = paddle_at {
            self.paddle_at = paddle;
        }

        if let Some(ball) = ball_at {
            self.ball_at = ball;
        }

        game_over
    }

    pub fn buffer_left(&mut self) {
        self.comp.input().queue(-1);
    }

    pub fn buffer_right(&mut self) {
        self.comp.input().queue(1);
    }

    pub fn buffer_neutral(&mut self) {
        self.comp.input().queue(0);
    }

    pub fn buffer_optimal(&mut self) {
        if self.paddle_at.x < self.ball_at.x {
            self.buffer_right();
        } else if self.paddle_at.x > self.ball_at.x {
            self.buffer_left();
        } else {
            self.buffer_neutral();
        }
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
