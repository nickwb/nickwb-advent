use crate::intcode::*;

const MY_INPUTS: [MemoryCell; 511] = [
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59, 76, 89, 106, 187, 268, 349, 430, 99999, 3, 9,
    1002, 9, 3, 9, 101, 2, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9, 1002, 9, 5, 9, 1001,
    9, 2, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 102, 4, 9, 9, 1001, 9, 3, 9, 4, 9, 99,
    3, 9, 101, 4, 9, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 3, 9, 101, 5, 9, 9, 1002, 9, 3, 9,
    4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
    101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
    9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9,
    1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9,
    4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101,
    2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4,
    9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2,
    9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
    9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2,
    9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
    9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9,
    9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
    9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2,
    9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 99,
];

fn part_one_try_phases(state: &VecStorage, phases: &[MemoryCell]) -> MemoryCell {
    phases.iter().fold(0 as MemoryCell, |o, p| {
        let result = run_io_intcode_program(state.clone(), &[*p, o]).unwrap();
        result
    })
}

fn part_one_find_max(state: &[MemoryCell]) -> MemoryCell {
    let state = Vec::from(state);
    get_permutations(&[0, 1, 2, 3, 4])
        .iter()
        .map(|p| part_one_try_phases(&state, p))
        .max()
        .unwrap()
}

// fn part_two_try_phases(phases: &[MemoryCell]) -> MemoryCell {
//     let make_a_computer = |phase| {
//         let input = intcode::BufferInput::new(2);
//         let output = intcode::RememberLastOutput::new();
//         let c = intcode::Computer::new(&mut MY_INPUTS.clone(), &mut input, &mut output);
//         input.queue(phase);
//         (input, output, c)
//     };

//     0
// }

pub fn run_day_seven() {
    let result = part_one_find_max(&MY_INPUTS);
    println!("Day 7, Part 1: {}", result);
}

fn get_permutations(values: &[MemoryCell]) -> Vec<[MemoryCell; 5]> {
    if values.len() != 5 {
        panic!("Wrong size");
    }

    let mut results = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let set = [values[a], values[b], values[c], values[d], values[e]];
                        results.push(set);
                    }
                }
            }
        }
    }

    results
}

#[test]
fn example_1() {
    assert_eq!(
        43210,
        part_one_find_max(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0])
    );
}

#[test]
fn example_2() {
    assert_eq!(
        54321,
        part_one_find_max(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ])
    );
}

#[test]
fn example_3() {
    assert_eq!(
        65210,
        part_one_find_max(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ])
    );
}

#[test]
fn actual_part_1() {
    assert_eq!(199988, part_one_find_max(&MY_INPUTS));
}

// 199988
