use nom::{
    branch::alt,
    character::complete::{self, space0},
    combinator::{all_consuming, map},
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use regex::internal::Input;

pub fn run_day_eighteen() {
    // let grid_one = inputs();
    // println!("Day 18, Part 1: {}", calculate_part_1(grid_one));
    // println!("Day 18, Part 2: {}", calculate_part_2(grid_two));
}

fn bracket_expr(i: &str) -> IResult<&str, i64> {
    delimited(complete::char('('), expr_parser, complete::char(')'))(i)
}

fn atom(i: &str) -> IResult<&str, i64> {
    let brackets_or_num = alt((bracket_expr, complete::i64));
    delimited(space0, brackets_or_num, space0)(i)
}

type Op = fn(i64, i64) -> i64;

fn op_parser(i: &str) -> IResult<&str, Op> {
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

fn expr_parser(i: &str) -> IResult<&str, i64> {
    // Get the first operand
    let (remaining, first) = atom(i)?;

    // Get pairs of operators and operands
    let part = pair(op_parser, atom);

    // Apply the pairs left to right
    fold_many0(part, move || first, |lhs, (op, rhs)| op(lhs, rhs))(remaining)
}

fn parse_and_evalute(expr: &str) -> i64 {
    let (_, o) = all_consuming(expr_parser)(expr).unwrap();
    o
}

// fn inputs() -> ActiveGrid {
//     let text = crate::util::read_file("inputs/day17.txt");
//     parse_initial_grid(&text)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(71, parse_and_evalute("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn example_2() {
        assert_eq!(26, parse_and_evalute("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn example_3() {
        assert_eq!(437, parse_and_evalute("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            12240,
            parse_and_evalute("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            13632,
            parse_and_evalute("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    // #[test]
    // fn actual_inputs() {
    //     let grid_one = inputs();
    //     let grid_two = grid_one.clone();
    //     assert_eq!(291, calculate_part_1(grid_one));
    //     assert_eq!(1524, calculate_part_2(grid_two));
    // }
}
