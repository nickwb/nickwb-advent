use std::collections::HashSet;

use crate::intcode::{Computer, MemoryCell, NoInput, OutputSink, StepResult};

mod path_find;

pub fn run_day_seventeen() {
    let part_1 = solve_first();

    println!("Day 17, Part 1: {}", part_1);
    println!("Day 17, Part 2: {}", 0);
}

// 4,L,4,L,4,L,4,L,4,L,12

// 10,L,6,L,4,L,4,L,12
// L,8,L,6,L,6,L,12
// R,10,L,12,R,6, R,10,L,12,R,6
// A,A,A,A,A,A,A,A

fn solve_first() -> usize {
    let program = inputs();
    let mut computer = Computer::new(program, NoInput, CameraBuffer::default());
    computer.enable_extra_memory();

    match computer.resume() {
        Ok(StepResult::Halt) => (),
        _ => panic!("Something didn't compute as expected"),
    };

    let camera = computer.output();
    eprintln!("{}", camera.to_string());

    let (width, height) = camera.width_and_height().expect("Can get width and height");
    let mut result = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let is_intersection = camera.is_scaffolded(x, y)
                && camera.is_scaffolded(x + 1, y)
                && camera.is_scaffolded(x - 1, y)
                && camera.is_scaffolded(x, y + 1)
                && camera.is_scaffolded(x, y - 1);

            if is_intersection {
                result += x * y;
            }
        }
    }

    result
}

#[derive(PartialEq)]
enum RobotState {
    Up,
    Down,
    Left,
    Right,
    Loose,
}

enum Observation {
    Empty,
    Scaffold,
    Robot { state: RobotState },
}

struct CameraBuffer {
    grid: Vec<Observation>,
    stride: Option<usize>,
}

impl CameraBuffer {
    fn width_and_height(&self) -> Option<(usize, usize)> {
        let stride = self.stride?;
        Some((stride, self.grid.len() / stride))
    }

    fn get(&self, x: usize, y: usize) -> Option<&Observation> {
        let stride = self.stride?;
        let idx = (stride * y) + x;
        self.grid.get(idx)
    }

    fn is_scaffolded(&self, x: usize, y: usize) -> bool {
        let here = self.get(x, y);
        match here {
            Some(Observation::Scaffold) => true,
            Some(Observation::Robot { state }) if state != &RobotState::Loose => true,
            _ => false,
        }
    }

    fn to_string(&self) -> String {
        let stride = self.stride.expect("Need a stride");
        let width = stride;
        let height = self.grid.len() / width;
        let mut str = String::with_capacity((width + 2) * height);
        for (idx, obs) in self.grid.iter().enumerate() {
            if (idx % stride) == 0 && str.len() != 0 {
                str.push_str("\r\n");
            }

            let symbol = match obs {
                Observation::Empty => ' ',
                Observation::Scaffold => '#',
                //Observation::Robot { state: _ } => '#'
                Observation::Robot {
                    state: RobotState::Up,
                } => '^',
                Observation::Robot {
                    state: RobotState::Down,
                } => 'V',
                Observation::Robot {
                    state: RobotState::Left,
                } => '<',
                Observation::Robot {
                    state: RobotState::Right,
                } => '>',
                Observation::Robot {
                    state: RobotState::Loose,
                } => '@',
            };
            str.push(symbol);
        }

        str
    }
}

impl Default for CameraBuffer {
    fn default() -> Self {
        Self {
            grid: Vec::new(),
            stride: None,
        }
    }
}

impl OutputSink for CameraBuffer {
    fn write(&mut self, value: MemoryCell) {
        let byte: u8 = value.try_into().expect("Should fit in a byte");
        let symbol = byte as char;

        match symbol {
            '.' => self.grid.push(Observation::Empty),
            '#' => self.grid.push(Observation::Scaffold),
            '\n' => {
                if let None = self.stride {
                    self.stride = Some(self.grid.len())
                }
            }
            '^' => self.grid.push(Observation::Robot {
                state: RobotState::Up,
            }),
            'V' => self.grid.push(Observation::Robot {
                state: RobotState::Down,
            }),
            '<' => self.grid.push(Observation::Robot {
                state: RobotState::Left,
            }),
            '>' => self.grid.push(Observation::Robot {
                state: RobotState::Right,
            }),
            'X' => self.grid.push(Observation::Robot {
                state: RobotState::Loose,
            }),
            _ => panic!("Unexpected symbol: {}", symbol),
        }
    }
}

fn inputs() -> Vec<MemoryCell> {
    crate::util::read_int_array("inputs/day17.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_inputs() {
        assert_eq!(7816, solve_first());
    }
}
