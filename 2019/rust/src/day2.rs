use crate::intcode::*;
use crate::util;
use rayon::prelude::*;

fn input() -> Vec<MemoryCell> {
    util::read_int_array("inputs/day2.txt")
}

fn find_required_values(input: Vec<MemoryCell>) -> isize {
    const TARGET_RESULT: MemoryCell = 19690720;
    let found = (0..=9999isize)
        .into_par_iter()
        .map(|x| {
            let i = x / 100;
            let j = x % 100;
            let mut inputs = input.clone();
            inputs[1] = i;
            inputs[2] = j;
            let result = run_basic_intcode_program(inputs, 0).unwrap();
            (x, result)
        })
        .find_any(|(_x, result)| *result == TARGET_RESULT)
        .unwrap();

    found.0
}

fn calculate_day_two() -> (isize, isize) {
    let input = input();
    let part_one = run_basic_intcode_program(input.clone(), 0).unwrap();
    let part_two = find_required_values(input);
    (part_one, part_two)
}

pub fn run_day_two() {
    let (part_one, part_two) = calculate_day_two();
    println!("Day 2, Part 1: {}", part_one);
    println!("Day 2, Part 2: {}", part_two);
}

#[test]
fn example_1() {
    assert_eq!(
        2,
        run_basic_intcode_program(slice_storage(&mut [1, 0, 0, 0, 99]), 0).unwrap()
    );
}

#[test]
fn example_2() {
    assert_eq!(
        6,
        run_basic_intcode_program(slice_storage(&mut [2, 3, 0, 3, 99]), 3).unwrap()
    );
}

#[test]
fn example_3() {
    assert_eq!(
        9801,
        run_basic_intcode_program(slice_storage(&mut [2, 4, 4, 5, 99, 0]), 5).unwrap()
    );
}

#[test]
fn example_4() {
    assert_eq!(
        30,
        run_basic_intcode_program(slice_storage(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]), 0).unwrap()
    );
}

#[test]
fn actual_day_1() {
    let (part_one, part_two) = calculate_day_two();
    assert_eq!(3706713, part_one);
    assert_eq!(8609, part_two);
}
