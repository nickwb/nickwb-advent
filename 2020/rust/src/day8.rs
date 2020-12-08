use std::collections::HashSet;

use regex::Regex;

pub fn run_day_eight() {
    let inputs = inputs();
    println!("Day 8, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 8, Part 2: {}", calculate_part_2(&inputs));
}

fn calculate_part_1(program: &Program) -> isize {
    match run_until_cycle_or_terminates(program) {
        RunResult::Cycled(x) => x,
        _ => panic!("This should be impossible in part 1"),
    }
}

fn calculate_part_2(program: &Program) -> isize {
    program
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instruction)| {
            let altered_instruction = match instruction.op {
                OpCode::Jmp => Some(Instruction {
                    op: OpCode::Nop,
                    argument: instruction.argument,
                }),
                // Check if switching this to a JMP would leave the valid bounds of the program
                OpCode::Nop if program.is_valid_ptr((i as isize) + instruction.argument) => {
                    Some(Instruction {
                        op: OpCode::Jmp,
                        argument: instruction.argument,
                    })
                }
                _ => None,
            };

            Some(ModifiedProgram::new(
                program,
                i as isize,
                altered_instruction?,
            ))
        })
        .find_map(|prog| match run_until_cycle_or_terminates(&prog) {
            RunResult::Cycled(_) => None,
            RunResult::Terminated(x) => Some(x),
        })
        .expect("Expected to find a suitable candidate")
}

enum RunResult {
    Terminated(isize),
    Cycled(isize),
}

fn run_until_cycle_or_terminates<P: InstructionSource>(program: &P) -> RunResult {
    let mut console = HandheldConsole::init(program);
    let mut visited: HashSet<isize> = HashSet::new();

    loop {
        if visited.contains(&console.instruction_ptr) {
            return RunResult::Cycled(console.accumulator);
        }

        if program.is_terminated(console.instruction_ptr) {
            return RunResult::Terminated(console.accumulator);
        }

        visited.insert(console.instruction_ptr);
        console.step();
    }
}

trait InstructionSource {
    fn get_instruction(&self, instruction_ptr: isize) -> &Instruction;
    fn is_valid_ptr(&self, instruction_ptr: isize) -> bool;
    fn is_terminated(&self, instruction_ptr: isize) -> bool;
}

impl InstructionSource for Program {
    fn get_instruction(&self, instruction_ptr: isize) -> &Instruction {
        self.instructions
            .get(instruction_ptr as usize)
            .expect("Outside of bounds")
    }

    fn is_valid_ptr(&self, instruction_ptr: isize) -> bool {
        instruction_ptr >= 0 && (instruction_ptr as usize) <= self.instructions.len()
    }

    fn is_terminated(&self, instruction_ptr: isize) -> bool {
        instruction_ptr > 0 && (instruction_ptr as usize) == self.instructions.len()
    }
}

impl<'prog> InstructionSource for ModifiedProgram<'prog> {
    fn get_instruction(&self, instruction_ptr: isize) -> &Instruction {
        if self.altered_instruction_ptr == instruction_ptr {
            &self.altered_instruction
        } else {
            self.source_program.get_instruction(instruction_ptr)
        }
    }

    fn is_valid_ptr(&self, instruction_ptr: isize) -> bool {
        self.source_program.is_valid_ptr(instruction_ptr)
    }

    fn is_terminated(&self, instruction_ptr: isize) -> bool {
        self.source_program.is_terminated(instruction_ptr)
    }
}

struct HandheldConsole<'prog, P: InstructionSource> {
    instruction_ptr: isize,
    accumulator: isize,
    program: &'prog P,
}

impl<'prog, P: InstructionSource> HandheldConsole<'prog, P> {
    fn init(program: &'prog P) -> Self {
        HandheldConsole {
            instruction_ptr: 0,
            accumulator: 0,
            program,
        }
    }

    fn step(&mut self) {
        let instruction = self.program.get_instruction(self.instruction_ptr);

        match instruction.op {
            OpCode::Acc => {
                self.accumulator += instruction.argument;
                self.instruction_ptr += 1;
            }
            OpCode::Jmp => {
                self.instruction_ptr += instruction.argument;
            }
            OpCode::Nop => {
                self.instruction_ptr += 1;
            }
        }
    }
}

fn inputs() -> Program {
    let text = crate::util::read_file("inputs/day8.txt");
    Program::parse(&text)
}

struct ModifiedProgram<'prog> {
    source_program: &'prog Program,
    altered_instruction_ptr: isize,
    altered_instruction: Instruction,
}

impl<'prog> ModifiedProgram<'prog> {
    fn new(
        program: &'prog Program,
        instruction_ptr: isize,
        instruction: Instruction,
    ) -> ModifiedProgram {
        ModifiedProgram {
            source_program: program,
            altered_instruction_ptr: instruction_ptr,
            altered_instruction: instruction,
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn parse(text: &str) -> Program {
        Program {
            instructions: text
                .lines()
                .map(|l| l.trim())
                .filter(|l| l.len() > 0)
                .map(|l| Instruction::parse(l).expect("valid instruction"))
                .collect(),
        }
    }
}

struct Instruction {
    op: OpCode,
    argument: isize,
}

lazy_static! {
    static ref INSTRUCTION_PATTERN: Regex = Regex::new(r"^(acc|jmp|nop) (\+|\-)(\d+)$").unwrap();
}

impl Instruction {
    fn parse(text: &str) -> Option<Instruction> {
        let captures = INSTRUCTION_PATTERN.captures(text)?;
        let op = match captures.get(1)?.as_str() {
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            "nop" => OpCode::Nop,
            _ => unreachable!("Regex"),
        };
        let sign = match captures.get(2)?.as_str() {
            "+" => 1isize,
            "-" => -1isize,
            _ => unreachable!("Regex"),
        };
        let argument = captures.get(3)?.as_str().parse::<isize>().ok()? * sign;
        Some(Instruction { op, argument })
    }
}

enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        ";
        let program = Program::parse(text);
        assert_eq!(5, calculate_part_1(&program));
        assert_eq!(8, calculate_part_2(&program));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(1487, calculate_part_1(&inputs));
        assert_eq!(1607, calculate_part_2(&inputs));
    }
}
