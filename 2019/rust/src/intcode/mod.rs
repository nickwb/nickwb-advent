mod computer;
mod io;

use computer::Computer;
use io::{BufferInput, NoInput, RememberLastOutput};

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
    EffectMismatch,
}

pub type IntCodeResult<T> = Result<T, IntCodeError>;

pub fn run_basic_intcode_program(
    state: ComputerState,
    final_addr: MemoryPointer,
) -> IntCodeResult<MemoryCell> {
    let mut input = NoInput;
    let mut output = RememberLastOutput::new();
    let mut computer = Computer::new(state, &mut input, &mut output);
    computer.run_until_halt()?;
    Ok(computer.get_memory_at(final_addr)?)
}

pub fn run_io_intcode_program(
    state: ComputerState,
    inputs: &[MemoryCell],
) -> IntCodeResult<MemoryCell> {
    let mut input = BufferInput::new(inputs.len());
    input.queue_many(inputs);

    let mut output = RememberLastOutput::new();
    let mut computer = Computer::new(state, &mut input, &mut output);
    computer.run_until_halt()?;
    Ok(output.last().expect("No output produced"))
}
