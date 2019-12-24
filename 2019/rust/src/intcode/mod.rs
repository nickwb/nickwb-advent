mod computer;
mod io;
mod storage;

pub use computer::*;
pub use io::*;
pub use storage::*;

pub type MemoryCell = isize;
pub type MemoryPointer = usize;

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

pub fn run_basic_intcode_program<S: Storage>(
    state: S,
    final_addr: MemoryPointer,
) -> IntCodeResult<MemoryCell> {
    let mut computer = Computer::new(state, NoInput, BufferOutput::new(0));
    computer.run_until_halt()?;
    Ok(computer.state().get(final_addr))
}

pub fn run_io_intcode_program<S: Storage>(
    state: S,
    inputs: &[MemoryCell],
) -> IntCodeResult<MemoryCell> {
    let mut computer = Computer::new(state, BufferInput::new(inputs.len()), BufferOutput::new(1));
    computer.input().queue_many(inputs);

    computer.run_until_halt()?;
    Ok(computer.output().last().expect("No output produced"))
}
