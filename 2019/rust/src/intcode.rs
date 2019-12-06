use std::convert::TryInto;

pub type MemoryCell = isize;
pub type MemoryPointer = usize;
pub type ComputerState<'a> = &'a mut [MemoryCell];

#[derive(Debug)]
pub enum IntCodeError {
    UnknownOpCode,
    ReadMemoryOutOfBounds,
    WriteMemoryOutOfBounds,
    InvalidParameterIndex,
    UnknownParameterType,
    OutputParameterInImmediateMode,
    MemoryCellIsInvalidPointer,
    ParameterOutputMismatch,
}

pub type IntCodeResult<T> = Result<T, IntCodeError>;

pub fn run_intcode_program(
    state: ComputerState,
    final_addr: MemoryPointer,
) -> IntCodeResult<MemoryCell> {
    let mut computer = Computer::from_state(state);
    computer.run_until_halt()?;
    Ok(computer.get_memory_at(final_addr)?)
}

struct Computer<'a> {
    state: ComputerState<'a>,
    program_counter: MemoryPointer,
}

enum StepResult {
    Halt,
    IncrementProgramCounter(MemoryPointer),
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(MemoryPointer),
    Immediate(MemoryCell),
}

#[derive(Debug)]
struct ParameterCounts {
    inputs: usize,
    has_output: bool,
}

impl ParameterCounts {
    fn total_cells(&self) -> MemoryPointer {
        if self.has_output {
            self.inputs + 1
        } else {
            self.inputs
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl OpCode {
    fn from_digits(digits: &str) -> IntCodeResult<OpCode> {
        Ok(match digits {
            "01" => OpCode::Add,
            "02" => OpCode::Multiply,
            "99" => OpCode::Halt,
            _ => {
                return Err(IntCodeError::UnknownOpCode);
            }
        })
    }

    fn parameter_count(&self) -> ParameterCounts {
        match self {
            OpCode::Add => ParameterCounts {
                inputs: 2,
                has_output: true,
            },
            OpCode::Multiply => ParameterCounts {
                inputs: 2,
                has_output: true,
            },
            OpCode::Halt => ParameterCounts {
                inputs: 0,
                has_output: false,
            },
        }
    }
}

#[derive(Debug)]
struct Operation {
    op_code: OpCode,
    parameter_count: ParameterCounts,
    parameters: Vec<Parameter>,
    output: Option<MemoryPointer>,
}

impl Computer<'_> {
    fn from_state<'a>(state: ComputerState<'a>) -> Computer<'a> {
        Computer {
            state: state,
            program_counter: 0,
        }
    }

    fn single_step(&mut self, operation: &Operation) -> IntCodeResult<StepResult> {
        if let OpCode::Halt = operation.op_code {
            return Ok(StepResult::Halt);
        }

        let get = |idx: usize| {
            let param = operation
                .parameters
                .get(idx)
                .ok_or(IntCodeError::InvalidParameterIndex)?;
            match param {
                Parameter::Immediate(val) => Ok(*val),
                Parameter::Position(addr) => self.get_memory_at(*addr),
            }
        };

        let result = match operation.op_code {
            OpCode::Add => Some(get(0)? + get(1)?),
            OpCode::Multiply => Some(get(0)? * get(1)?),
            _ => None,
        };

        match (
            result,
            operation.output,
            operation.parameter_count.has_output,
        ) {
            (None, None, false) => Ok(()),
            (Some(value), Some(addr), true) => self.set_memory_at(addr, value),
            _ => Err(IntCodeError::ParameterOutputMismatch),
        }?;

        Ok(StepResult::IncrementProgramCounter(
            // One extra for the op_code cell
            operation.parameter_count.total_cells() + 1,
        ))
    }

    fn read_op(&self, from: MemoryPointer) -> IntCodeResult<Operation> {
        let op = self.get_memory_at(from)?;
        let digits = format!("{:05}", op);
        assert_eq!(5, digits.len());

        let op_code = OpCode::from_digits(&digits[3..5])?;
        let parameter_counts = op_code.parameter_count();

        // println!(
        //     "Op: {:?}, inputs: {}, has_output: {}",
        //     op_code, parameter_counts.inputs, parameter_counts.has_output
        // );

        // Make a positional parameter by reading a memory location from the given index
        let make_positional = |idx: usize| {
            let location: MemoryPointer = self
                .get_memory_at(from + 1 + idx)?
                .try_into()
                .map_err(|_| IntCodeError::MemoryCellIsInvalidPointer)?;
            Ok(Parameter::Position(location))
        };

        // Make an immediate parameter by reading a value from the given index
        let make_immediate = |idx: usize| {
            let value = self.get_memory_at(from + 1 + idx)?;
            Ok(Parameter::Immediate(value))
        };

        // Build a parameter by determining its type and reading its location/value
        let build_parameter = |idx: usize| {
            if idx + 1 > parameter_counts.inputs {
                return Err(IntCodeError::InvalidParameterIndex);
            }

            let mode_digit = 3 - idx;
            let mode = &digits[mode_digit..mode_digit + 1];
            match mode {
                "0" => make_positional(idx),
                "1" => make_immediate(idx),
                _ => {
                    return Err(IntCodeError::UnknownParameterType);
                }
            }
        };

        let parameters = (0..parameter_counts.inputs)
            .map(build_parameter)
            .collect::<IntCodeResult<Vec<_>>>()?;

        let output = if parameter_counts.has_output {
            // If we have an output address, find it after the input parameters
            match make_positional(parameter_counts.inputs) {
                Ok(Parameter::Position(p)) => Some(p),
                Ok(Parameter::Immediate(_)) => {
                    return Err(IntCodeError::OutputParameterInImmediateMode)
                }
                Err(e) => return Err(e),
            }
        } else {
            None
        };

        let operation = Operation {
            op_code: op_code,
            parameter_count: parameter_counts,
            parameters: parameters,
            output: output,
        };

        Ok(operation)
    }

    fn run_until_halt(&mut self) -> IntCodeResult<()> {
        loop {
            let op = self.read_op(self.program_counter)?;
            let step = self.single_step(&op)?;
            match step {
                StepResult::Halt => {
                    return Ok(());
                }
                StepResult::IncrementProgramCounter(inc) => self.program_counter += inc,
            }
        }
    }

    fn get_memory_at(&self, index: MemoryPointer) -> IntCodeResult<MemoryCell> {
        if index + 1 > self.state.len() {
            Err(IntCodeError::ReadMemoryOutOfBounds)
        } else {
            Ok(self.state[index])
        }
    }

    fn set_memory_at(&mut self, index: MemoryPointer, value: MemoryCell) -> IntCodeResult<()> {
        if index + 1 > self.state.len() {
            Err(IntCodeError::WriteMemoryOutOfBounds)
        } else {
            self.state[index] = value;
            Ok(())
        }
    }
}

#[test]
fn parse_operation() {
    let state: ComputerState = &mut [1, 2, 3, 4];
    let computer = Computer::from_state(state);
    let operation = computer.read_op(0).unwrap();

    assert_eq!(OpCode::Add, operation.op_code);
    assert_eq!(Parameter::Position(2), operation.parameters[0]);
    assert_eq!(Parameter::Position(3), operation.parameters[1]);
    assert_eq!(4, operation.output.unwrap());
}

#[test]
fn simple_program() {
    let state: ComputerState = &mut [1, 0, 0, 0, 99];
    assert_eq!(2, run_intcode_program(state, 0).unwrap());
}
