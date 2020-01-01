use crate::intcode::*;
use crate::util;

fn input() -> Vec<MemoryCell> {
    util::read_int_array("inputs/day5.txt")
}

fn calculate_day_five() -> (MemoryCell, MemoryCell) {
    let input = input();
    let part_one = run_io_intcode_program(input.clone(), &[1]).unwrap();
    let part_two = run_io_intcode_program(input.clone(), &[5]).unwrap();
    (part_one, part_two)
}

pub fn run_day_five() {
    let (part_one, part_two) = calculate_day_five();
    println!("Day 5, Part 1: {}", part_one);
    println!("Day 5, Part 2: {}", part_two);
}

#[test]
fn actual_day_5() {
    let (part_one, part_two) = calculate_day_five();
    assert_eq!(12440243, part_one);
    assert_eq!(15486302, part_two);
}
