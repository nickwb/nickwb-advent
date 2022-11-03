use nom::{
    branch::alt,
    character::complete::{self, space0},
    combinator::{all_consuming, map},
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use once_cell::sync::Lazy;

pub fn run_day_eighteen() {
    let inputs = inputs();
    println!("Day 18, Part 1: {}", calculate_part_1(&inputs));
    // println!("Day 18, Part 2: {}", calculate_part_2(grid_two));
}

pub fn calculate_part_1<T: AsRef<str>>(lines: &[T]) -> i64 {
    lines.iter().map(|s| parse_and_evaluate(s.as_ref())).sum()
}

type Op = fn(i64, i64) -> i64;

fn bracketed_expression(i: &str) -> IResult<&str, i64> {
    delimited(complete::char('('), expression, complete::char(')'))(i)
}

fn atomic_operand(i: &str) -> IResult<&str, i64> {
    let brackets_or_num = alt((bracketed_expression, complete::i64));
    delimited(space0, brackets_or_num, space0)(i)
}

fn operator(i: &str) -> IResult<&str, Op> {
    let symbol = alt((complete::char('+'), complete::char('*')));

    fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    fn multiply(a: i64, b: i64) -> i64 {
        a * b
    }

    map(symbol, |c| match c {
        '+' => add,
        '*' => multiply,
        _ => unreachable!("Not parsed"),
    })(i)
}

fn expression(i: &str) -> IResult<&str, i64> {
    // Get the first operand
    let (remaining, first) = atomic_operand(i)?;

    // Get pairs of operators and operands
    let part = pair(operator, atomic_operand);

    // Apply the pairs left to right
    fold_many0(part, move || first, |lhs, (op, rhs)| op(lhs, rhs))(remaining)
}

fn parse_and_evaluate(expr: &str) -> i64 {
    let (_, o) = all_consuming(expression)(expr).unwrap();
    o
}

fn inputs() -> Vec<String> {
    let text = crate::util::read_file("inputs/day18.txt");
    text.lines().map(|s| s.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(71, parse_and_evaluate("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn example_2() {
        assert_eq!(26, parse_and_evaluate("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn example_3() {
        assert_eq!(437, parse_and_evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            12240,
            parse_and_evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            13632,
            parse_and_evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(4491283311856, calculate_part_1(&inputs));
        // assert_eq!(1524, calculate_part_2(grid_two));
    }
}
