use std::collections::HashMap;

type Cell = usize;
type StateMap = HashMap<Cell, Cell>;

fn do_op<F: FnOnce(Cell, Cell) -> Cell>(state: &mut StateMap, from: Cell, func: F) -> Cell {
    let addr_a = state[&(from + 1)];
    let addr_b = state[&(from + 2)];
    let addr_z = state[&(from + 3)];
    let a = state[&addr_a];
    let b = state[&addr_b];
    let z = func(a, b);
    *state.entry(addr_z).or_default() = z;
    from + 4
}

fn do_compute(state: &mut StateMap, final_state: Cell) -> Cell {
    let mut ip: Cell = 0;

    loop {
        let opcode = state[&ip];
        match opcode {
            1 => {
                ip = do_op(state, ip, |a, b| a + b);
            }
            2 => {
                ip = do_op(state, ip, |a, b| a * b);
            }
            99 => {
                return state[&final_state];
            }
            _ => {
                panic!("Unknown opcode");
            }
        }
    }
}

fn run_machine(state: &[Cell], final_state: Cell) -> Cell {
    let mut map: StateMap = HashMap::new();
    for (idx, elem) in state.iter().enumerate() {
        map.insert(idx as Cell, *elem);
    }
    do_compute(&mut map, final_state)
}

const MY_INPUTS: [Cell; 129] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 6, 19, 23, 1, 13, 23, 27, 1,
    6, 27, 31, 1, 31, 10, 35, 1, 35, 6, 39, 1, 39, 13, 43, 2, 10, 43, 47, 1, 47, 6, 51, 2, 6, 51,
    55, 1, 5, 55, 59, 2, 13, 59, 63, 2, 63, 9, 67, 1, 5, 67, 71, 2, 13, 71, 75, 1, 75, 5, 79, 1,
    10, 79, 83, 2, 6, 83, 87, 2, 13, 87, 91, 1, 9, 91, 95, 1, 9, 95, 99, 2, 99, 9, 103, 1, 5, 103,
    107, 2, 9, 107, 111, 1, 5, 111, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2, 0, 14, 0,
];

pub fn run_day_two() {
    let part_one: Cell = run_machine(&MY_INPUTS, 0);
    println!("Day Two. The part one result is: {}", part_one);

    #[cfg(run_slowly)]
    for i in 0..99 {
        for j in 0..99 {
            let r: &[Cell] = &MY_INPUTS;
            let mut copy: Vec<Cell> = r.into();

            copy[1] = i;
            copy[2] = j;
            let result = run_machine(&copy, 0);

            if result == 19690720 {
                let part_two = 100 * i + j;
                println!("Day Two. The part two result is: {}", part_two);
                return;
            }
        }
    }
}

#[test]
fn example_1() {
    assert_eq!(2, run_machine(&[1, 0, 0, 0, 99], 0));
}

#[test]
fn example_2() {
    assert_eq!(6, run_machine(&[2, 3, 0, 3, 99], 3));
}

#[test]
fn example_3() {
    assert_eq!(9801, run_machine(&[2, 4, 4, 5, 99, 0], 5));
}

#[test]
fn example_4() {
    assert_eq!(30, run_machine(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 0));
}
