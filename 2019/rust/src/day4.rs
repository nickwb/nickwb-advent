use std::fmt::Write;

fn is_valid(digits: &str, enforce_doubles_only: bool) -> bool {
    if digits.len() != 6 {
        return false;
    }

    struct State {
        last_char: char,
        repeat_count: usize,
        has_double: bool,
        is_strictly_increasing: bool,
    }

    let initial = State {
        last_char: '!',
        repeat_count: 0,
        has_double: false,
        is_strictly_increasing: true,
    };

    let mut state = digits.chars().fold(initial, |mut state, next| {
        let is_match = next == state.last_char;
        state.has_double = state.has_double
            || (!enforce_doubles_only && is_match) // This digit is at least a double
            || (enforce_doubles_only && !is_match && state.repeat_count == 1); // The previous digit was a double
        state.repeat_count = if is_match { state.repeat_count + 1 } else { 0 };
        state.is_strictly_increasing = state.is_strictly_increasing && state.last_char <= next;
        state.last_char = next;
        state
    });

    // Check if that last digit was a valid double
    state.has_double = state.has_double || (enforce_doubles_only && state.repeat_count == 1);

    state.has_double && state.is_strictly_increasing
}

fn valids_between(start: i32, end: i32, enforce_doubles_only: bool) -> usize {
    let mut buffer = String::with_capacity(6);

    (start..=end)
        .filter(|x| {
            buffer.clear();
            write!(&mut buffer, "{}", x).unwrap();
            is_valid(&buffer, enforce_doubles_only)
        })
        .count()
}

fn calculate_day_four() -> (usize, usize) {
    let start = 146810;
    let end = 612564;
    (
        valids_between(start, end, false),
        valids_between(start, end, true),
    )
}

pub fn run_day_four() {
    let (part_one, part_two) = calculate_day_four();
    println!("Day 4, Part 1: {}", part_one);
    println!("Day 4, Part 2: {}", part_two);
}

#[test]
fn example_1() {
    assert_eq!(true, is_valid("111111", false));
    assert_eq!(false, is_valid("223450", false));
    assert_eq!(false, is_valid("123789", false));
}

#[test]
fn example_2() {
    assert_eq!(true, is_valid("112233", true));
    assert_eq!(false, is_valid("123444", true));
    assert_eq!(true, is_valid("111122", true));
}

#[test]
fn actual_day_3() {
    let (part_one, part_two) = calculate_day_four();
    assert_eq!(1748, part_one);
    assert_eq!(1180, part_two);
}
