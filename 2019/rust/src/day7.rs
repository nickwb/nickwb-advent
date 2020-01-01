use crate::intcode::*;
use crate::util;

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

fn part_two_try_phases(state: &[MemoryCell], phases: &[MemoryCell]) -> MemoryCell {
    let make_a_computer = |&phase| {
        let mut c = Computer::new(Vec::from(state), BufferInput::new(2), BufferOutput::new(1));
        c.input().queue(phase);
        c
    };

    let mut computers: Vec<Computer<VecStorage, BufferInput, BufferOutput>> =
        phases.iter().map(make_a_computer).collect();

    computers[0].input().queue(0);

    let mut i = 0;
    let mut result: Option<MemoryCell> = None;

    loop {
        let j = if i < computers.len() - 1 { i + 1 } else { 0 };
        let outputs: Vec<MemoryCell>;

        {
            let this_computer = computers.get_mut(i).unwrap();
            this_computer.resume().unwrap();

            let output = this_computer.output();

            if j == 0 {
                result = output.last().or(result);
            }

            outputs = output.pop_all();
        }

        {
            let next_computer = computers.get_mut(j).unwrap();

            if next_computer.has_halted() {
                return result.expect("Got to the end of the chain, but there was no result");
            }

            next_computer.input().queue_many(&outputs);
        }

        i = j;
    }
}

fn part_two_find_max(state: &[MemoryCell]) -> MemoryCell {
    let state = Vec::from(state);
    get_permutations(&[5, 6, 7, 8, 9])
        .iter()
        .map(|p| part_two_try_phases(&state, p))
        .max()
        .unwrap()
}

fn input() -> Vec<MemoryCell> {
    util::read_int_array("inputs/day7.txt")
}

fn calculate_day_seven() -> (MemoryCell, MemoryCell) {
    let input = input();
    let part_one = part_one_find_max(&input);
    let part_two = part_two_find_max(&input);
    (part_one, part_two)
}

pub fn run_day_seven() {
    let (part_one, part_two) = calculate_day_seven();
    println!("Day 7, Part 1: {}", part_one);
    println!("Day 7, Part 2: {}", part_two);
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
fn example_4() {
    assert_eq!(
        139629729,
        part_two_find_max(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ])
    );
}

#[test]
fn example_5() {
    assert_eq!(
        18216,
        part_two_find_max(&[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ])
    );
}

#[test]
fn actual_day_7() {
    let (part_one, part_two) = calculate_day_seven();
    assert_eq!(199988, part_one);
    assert_eq!(17519904, part_two);
}
