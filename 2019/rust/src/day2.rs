use crate::intcode;

const MY_INPUTS: [intcode::MemoryCell; 129] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 1, 13, 23, 27, 1,
    6, 27, 31, 1, 31, 10, 35, 1, 35, 6, 39, 1, 39, 13, 43, 2, 10, 43, 47, 1, 47, 6, 51, 2, 6, 51,
    55, 1, 5, 55, 59, 2, 13, 59, 63, 2, 63, 9, 67, 1, 5, 67, 71, 2, 13, 71, 75, 1, 75, 5, 79, 1,
    10, 79, 83, 2, 6, 83, 87, 2, 13, 87, 91, 1, 9, 91, 95, 1, 9, 95, 99, 2, 99, 9, 103, 1, 5, 103,
    107, 2, 9, 107, 111, 1, 5, 111, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2, 0, 14, 0,
];

const TARGET_RESULT: intcode::MemoryCell = 19690720;

pub fn run_day_two() {
    let part_one = intcode::run_intcode_program(&mut MY_INPUTS.clone(), 0).unwrap();
    println!("Day Two. The part one result is: {}", part_one);

    let mut i = 0;
    let mut j = 0;

    loop {
        let state: intcode::ComputerState = &mut MY_INPUTS.clone();
        state[1] = i;
        state[2] = j;
        let result = intcode::run_intcode_program(state, 0).unwrap();

        if result == TARGET_RESULT {
            let part_two = (100 * i) + j;
            println!("Day Two. The part two result is: {}", part_two);
            return;
        }

        if result > TARGET_RESULT {
            i += 1;
            j = 0;
        } else if j < 99 {
            j += 1;
        } else {
            i += 1;
            j = 0;
        }
    }
}

#[test]
fn example_1() {
    assert_eq!(
        2,
        intcode::run_intcode_program(&mut [1, 0, 0, 0, 99], 0).unwrap()
    );
}

#[test]
fn example_2() {
    assert_eq!(
        6,
        intcode::run_intcode_program(&mut [2, 3, 0, 3, 99], 3).unwrap()
    );
}

#[test]
fn example_3() {
    assert_eq!(
        9801,
        intcode::run_intcode_program(&mut [2, 4, 4, 5, 99, 0], 5).unwrap()
    );
}

#[test]
fn example_4() {
    assert_eq!(
        30,
        intcode::run_intcode_program(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99], 0).unwrap()
    );
}
