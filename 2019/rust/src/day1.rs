use crate::util;

fn get_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

fn get_total_fuel(module_mass: f64) -> f64 {
    let mut total: f64 = 0.0;
    let mut mass: f64 = get_fuel(module_mass);
    while mass > 0.0 {
        total = total + mass;
        mass = get_fuel(mass);
    }

    total
}

fn input() -> Vec<f64> {
    util::read_int_array("inputs/day1.txt")
}

fn calculate() -> (f64, f64) {
    let input = input();
    let part_one: f64 = input.iter().copied().map(|i| get_fuel(i)).sum();
    let part_two: f64 = input.iter().copied().map(|i| get_total_fuel(i)).sum();
    (part_one, part_two)
}

pub fn run_day_one() {
    let (part_one, part_two) = calculate();
    println!("Day 1, Part 1: {}", part_one);
    println!("Day 1, Part 2: {}", part_two);
}

#[test]
fn example_1() {
    assert_eq!(2.0, get_fuel(12.0));
}

#[test]
fn example_2() {
    assert_eq!(2.0, get_fuel(14.0));
}

#[test]
fn example_3() {
    assert_eq!(654.0, get_fuel(1969.0));
}

#[test]
fn example_4() {
    assert_eq!(33583.0, get_fuel(100756.0));
}

#[test]
fn example_5() {
    assert_eq!(2.0, get_total_fuel(14.0));
}

#[test]
fn example_6() {
    assert_eq!(966.0, get_total_fuel(1969.0));
}

#[test]
fn example_7() {
    assert_eq!(50346.0, get_total_fuel(100756.0));
}

#[test]
fn actual_day_1() {
    let (part_one, part_two) = calculate();
    assert_eq!(3412207f64, part_one);
    assert_eq!(5115436f64, part_two);
}
