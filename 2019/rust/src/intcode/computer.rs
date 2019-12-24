use super::io::{InputSource, OutputSink};
use super::storage::Storage;
use super::{IntCodeError, IntCodeResult, MemoryCell, MemoryPointer};
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Write;

pub struct Computer<S: Storage, I: InputSource, O: OutputSink> {
    state: S,
    program_counter: MemoryPointer,
    relative_base: MemoryCell,
    input: I,
    output: O,
    has_halted: bool,
    extra_memory: Option<HashMap<MemoryPointer, MemoryCell>>,
}

pub enum StepResult {
    Halt,
    Continue,
    WaitingOnInput,
}

fn debug_log(args: std::fmt::Arguments) {
    //let mut output = std::io::sink(); // std::io::stdout
    let mut output = std::io::stdout(); // std::io::stdout
    writeln!(&mut output, "{}", args).expect("Failed to write debug log");
}

#[derive(Debug)]
enum Effect {
    NoOp,
    StoreValue(MemoryCell),
    OutputValue(MemoryCell),
    Jump(MemoryPointer),
    SetRelativeBase(MemoryCell),
}

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(MemoryPointer),
    Immediate(MemoryCell),
    Relative(MemoryCell),
}

#[derive(Debug)]
struct ParameterTypes {
    inputs: usize,
    has_output_parameter: bool,
}

impl ParameterTypes {
    fn total_cells(&self) -> MemoryPointer {
        match self.has_output_parameter {
            true => self.inputs + 1,
            false => self.inputs,
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    SetRelativeBase,
    Halt,
}

impl OpCode {
    fn from_digits(digits: &str) -> IntCodeResult<OpCode> {
        Ok(match digits {
            "01" => OpCode::Add,
            "02" => OpCode::Multiply,
            "03" => OpCode::Input,
            "04" => OpCode::Output,
            "05" => OpCode::JumpIfTrue,
            "06" => OpCode::JumpIfFalse,
            "07" => OpCode::LessThan,
            "08" => OpCode::Equals,
            "09" => OpCode::SetRelativeBase,
            "99" => OpCode::Halt,
            _ => {
                return Err(IntCodeError::UnknownOpCode);
            }
        })
    }

    fn parameter_types(&self) -> ParameterTypes {
        match self {
            OpCode::Add => ParameterTypes {
                inputs: 2,
                has_output_parameter: true,
            },
            OpCode::Multiply => ParameterTypes {
                inputs: 2,
                has_output_parameter: true,
            },
            OpCode::Input => ParameterTypes {
                inputs: 0,
                has_output_parameter: true,
            },
            OpCode::Output => ParameterTypes {
                inputs: 1,
                has_output_parameter: false,
            },
            OpCode::JumpIfTrue => ParameterTypes {
                inputs: 2,
                has_output_parameter: false,
            },
            OpCode::JumpIfFalse => ParameterTypes {
                inputs: 2,
                has_output_parameter: false,
            },
            OpCode::LessThan => ParameterTypes {
                inputs: 2,
                has_output_parameter: true,
            },
            OpCode::Equals => ParameterTypes {
                inputs: 2,
                has_output_parameter: true,
            },
            OpCode::SetRelativeBase => ParameterTypes {
                inputs: 1,
                has_output_parameter: false,
            },
            OpCode::Halt => ParameterTypes {
                inputs: 0,
                has_output_parameter: false,
            },
        }
    }
}

#[derive(Debug)]
struct Operation {
    op_code: OpCode,
    parameter_types: ParameterTypes,
    parameters: Vec<Parameter>,
    output_address: Option<MemoryPointer>,
}

fn cast_cell_to_pointer(value: MemoryCell) -> IntCodeResult<MemoryPointer> {
    Ok(value
        .try_into()
        .map_err(|_| IntCodeError::MemoryCellIsInvalidPointer)?)
}

impl Operation {
    fn get_program_counter_increment(&self) -> MemoryPointer {
        1 + self.parameter_types.total_cells()
    }
}

impl<S: Storage, I: InputSource, O: OutputSink> Computer<S, I, O> {
    pub fn new(state: S, input: I, output: O) -> Computer<S, I, O> {
        Computer {
            state: state,
            program_counter: 0,
            relative_base: 0,
            has_halted: false,
            input,
            output,
            extra_memory: None,
        }
    }

    pub fn enable_extra_memory(&mut self) {
        self.extra_memory = Some(HashMap::new());
    }

    pub fn input(&mut self) -> &mut I {
        &mut self.input
    }

    pub fn output(&mut self) -> &mut O {
        &mut self.output
    }

    pub fn state(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn has_halted(&self) -> bool {
        self.has_halted
    }

    fn increment_for_operation(&mut self, operation: &Operation) {
        let increment = operation.get_program_counter_increment();
        self.program_counter += increment;
    }

    fn get_input(&mut self) -> Option<MemoryCell> {
        let result = self.input.next();
        match &result {
            Some(v) => debug_log(format_args!("Read input: {}", *v)),
            None => debug_log(format_args!("No input available")),
        }

        result
    }

    fn put_output(&mut self, value: MemoryCell) {
        debug_log(format_args!("Writing output: {}", value));
        self.output.write(value)
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
                Parameter::Relative(offset) => {
                    debug_log(format_args!(
                        "Relative Deref: {} + {}",
                        self.relative_base, *offset
                    ));
                    self.get_memory_at(cast_cell_to_pointer(self.relative_base + *offset)?)
                }
            }
        };

        let result = match operation.op_code {
            OpCode::Add => Effect::StoreValue(get(0)? + get(1)?),
            OpCode::Multiply => Effect::StoreValue(get(0)? * get(1)?),
            OpCode::Input => match self.get_input() {
                Some(v) => Effect::StoreValue(v),
                None => return Ok(StepResult::WaitingOnInput),
            },
            OpCode::Output => Effect::OutputValue(get(0)?),
            OpCode::JumpIfTrue => {
                if get(0)? == 0 {
                    Effect::NoOp
                } else {
                    Effect::Jump(cast_cell_to_pointer(get(1)?)?)
                }
            }
            OpCode::JumpIfFalse => {
                if get(0)? == 0 {
                    Effect::Jump(cast_cell_to_pointer(get(1)?)?)
                } else {
                    Effect::NoOp
                }
            }
            OpCode::LessThan => {
                if get(0)? < get(1)? {
                    Effect::StoreValue(1)
                } else {
                    Effect::StoreValue(0)
                }
            }
            OpCode::Equals => {
                if get(0)? == get(1)? {
                    Effect::StoreValue(1)
                } else {
                    Effect::StoreValue(0)
                }
            }
            OpCode::SetRelativeBase => Effect::SetRelativeBase(get(0)?),
            _ => Effect::NoOp,
        };

        debug_log(format_args!("Effect: {:?}", result));

        let effect_states = (result, operation.output_address);
        match effect_states {
            (Effect::NoOp, None) => {
                self.increment_for_operation(operation);
                Ok(StepResult::Continue)
            }
            (Effect::StoreValue(value), Some(addr)) => {
                self.set_memory_at(addr, value)?;
                self.increment_for_operation(operation);
                Ok(StepResult::Continue)
            }
            (Effect::OutputValue(value), None) => {
                self.put_output(value);
                self.increment_for_operation(operation);
                Ok(StepResult::Continue)
            }
            (Effect::Jump(addr), None) => {
                self.program_counter = addr;
                Ok(StepResult::Continue)
            }
            (Effect::SetRelativeBase(offset), None) => {
                self.relative_base += offset;
                debug_log(format_args!("Relative Base = {}", self.relative_base));
                self.increment_for_operation(operation);
                Ok(StepResult::Continue)
            }
            _ => Err(IntCodeError::EffectMismatch),
        }
    }

    fn read_op(&self, from: MemoryPointer) -> IntCodeResult<Operation> {
        let op = self.get_memory_at(from)?;
        let digits = format!("{:05}", op);
        assert_eq!(5, digits.len());

        let op_code = OpCode::from_digits(&digits[3..5])?;
        let parameter_types = op_code.parameter_types();

        // Make a positional parameter by reading a memory location from the given index
        let make_positional = |idx: usize| {
            let location: MemoryPointer =
                cast_cell_to_pointer(self.get_memory_at(from + 1 + idx)?)?;
            Ok(Parameter::Position(location))
        };

        // Make an immediate parameter by reading a value from the given index
        let make_immediate = |idx: usize| {
            let value = self.get_memory_at(from + 1 + idx)?;
            Ok(Parameter::Immediate(value))
        };

        // Make a relative parameter by reading a memory offset from the given index
        let make_relative = |idx: usize| {
            let offset: MemoryCell = self.get_memory_at(from + 1 + idx)?;
            Ok(Parameter::Relative(offset))
        };

        // Build a parameter by determining its type and reading its location/value
        let build_parameter = |idx: usize| {
            if idx + 1 > parameter_types.inputs {
                return Err(IntCodeError::InvalidParameterIndex);
            }

            let mode_digit = 2 - idx;
            let mode = &digits[mode_digit..=mode_digit];
            match mode {
                "0" => make_positional(idx),
                "1" => make_immediate(idx),
                "2" => make_relative(idx),
                _ => {
                    return Err(IntCodeError::UnknownParameterType);
                }
            }
        };

        let parameters = (0..parameter_types.inputs)
            .map(build_parameter)
            .collect::<IntCodeResult<Vec<_>>>()?;

        let output = if parameter_types.has_output_parameter {
            // If we have an output address, find it immediately after the input parameters
            match make_positional(parameter_types.inputs) {
                Ok(Parameter::Position(p)) => Some(p),
                Ok(Parameter::Relative(_offset)) => {
                    return Err(IntCodeError::OutputParameterInRelativeMode)
                }
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
            parameter_types: parameter_types,
            parameters: parameters,
            output_address: output,
        };

        debug_log(format_args!(
            "Operation #{} [{}] is {:?}",
            from, digits, operation
        ));

        Ok(operation)
    }

    pub fn resume(&mut self) -> IntCodeResult<StepResult> {
        if self.has_halted {
            return Ok(StepResult::Halt);
        }

        loop {
            let op = self.read_op(self.program_counter)?;
            let step = self.single_step(&op)?;
            match step {
                StepResult::Halt => {
                    self.has_halted = true;
                    return Ok(StepResult::Halt);
                }
                StepResult::WaitingOnInput => {
                    return Ok(StepResult::WaitingOnInput);
                }
                StepResult::Continue => (),
            }
        }
    }

    pub fn run_until_halt(&mut self) -> IntCodeResult<()> {
        match self.resume()? {
            StepResult::Halt => {
                return Ok(());
            }
            StepResult::WaitingOnInput => panic!("Computer is blocked on input"),
            StepResult::Continue => panic!("Resume stopped on continue"),
        }
    }

    fn get_memory_at(&self, index: MemoryPointer) -> IntCodeResult<MemoryCell> {
        if index + 1 > self.state.size() {
            match &self.extra_memory {
                Some(mem) => {
                    let value = *mem.get(&index).unwrap_or(&0);
                    Ok(value)
                }
                None => Err(IntCodeError::ReadMemoryOutOfBounds),
            }
        } else {
            Ok(self.state.get(index))
        }
    }

    fn set_memory_at(&mut self, index: MemoryPointer, value: MemoryCell) -> IntCodeResult<()> {
        if index + 1 > self.state.size() {
            match &mut self.extra_memory {
                Some(mem) => {
                    mem.entry(index).and_modify(|e| *e = value).or_insert(value);
                    Ok(())
                }
                None => Err(IntCodeError::WriteMemoryOutOfBounds),
            }
        } else {
            self.state.put(index, value);
            Ok(())
        }
    }
}
