use crate::intcode::*;
use crate::util;

fn input() -> Vec<MemoryCell> {
    util::read_int_array("inputs/day9.txt")
}

fn run_with_input(code: Vec<MemoryCell>, input: MemoryCell) -> Vec<MemoryCell> {
    let mut computer = Computer::new(code, BufferInput::new(1), BufferOutput::new(1));
    computer.enable_extra_memory();
    computer.input().queue(input);
    computer.run_until_halt().unwrap();
    let output = computer.output();
    output.pop_all()
}

fn calculate_day_nine() -> (Vec<MemoryCell>, Option<Vec<MemoryCell>>) {
    let code = input();
    let part_1 = run_with_input(code.clone(), 1);
    let part_2: Option<Vec<MemoryCell>> = None;
    #[cfg(slow_problems)]
    let part_2 = Some(run_with_input(code.clone(), 2));
    (part_1, part_2)
}

pub fn run_day_nine() {
    let (part_one, part_two) = calculate_day_nine();
    println!("Day 9, Part 1: {:?}", part_one);
    println!("Day 9, Part 2: {:?}", part_two);
}

#[test]
fn example_1() {
    let state = &mut [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut computer = Computer::new(slice_storage(state), NoInput, BufferOutput::new(16));
    computer.enable_extra_memory();
    computer.run_until_halt().unwrap();
    let output = computer.output();
    assert_eq!(
        vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99),
        output.pop_all()
    );
}

#[test]
fn example_2() {
    let state = &mut [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut computer = Computer::new(slice_storage(state), NoInput, BufferOutput::new(1));
    computer.run_until_halt().unwrap();
    let result = computer.output().last().unwrap();
    assert_eq!(1219070632396864, result);
}

#[test]
fn example_3() {
    let state = &mut [104, 1125899906842624, 99];
    let mut computer = Computer::new(slice_storage(state), NoInput, BufferOutput::new(1));
    computer.run_until_halt().unwrap();
    let result = computer.output().last().unwrap();
    assert_eq!(1125899906842624, result);
}

#[test]
fn actual_day_9() {
    let (part_one, _part_two) = calculate_day_nine();
    assert_eq!(vec!(3601950151), part_one);
    #[cfg(slow_problems)]
    assert_eq!(vec!(64236), _part_two.unwrap());
}
