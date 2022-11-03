use nom::{
    branch::alt,
    character::complete::{self, space0},
    combinator::{all_consuming, map},
    multi::many0,
    sequence::delimited,
    IResult,
};

pub fn run_day_eighteen() {
    let inputs = inputs();
    println!("Day 18, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 18, Part 2: {}", calculate_part_2(&inputs));
}

pub fn calculate_part_1<T: AsRef<str>>(lines: &[T]) -> i64 {
    lines
        .iter()
        .map(|s| parse_and_evaluate(s.as_ref(), false))
        .sum()
}

pub fn calculate_part_2<T: AsRef<str>>(lines: &[T]) -> i64 {
    lines
        .iter()
        .map(|s| parse_and_evaluate(s.as_ref(), true))
        .sum()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
enum Ast {
    Value(i64),
    Op(Op),
    Expression(Vec<Ast>),
}

fn bracketed_expression(i: &str) -> IResult<&str, Ast> {
    delimited(complete::char('('), expression, complete::char(')'))(i)
}

fn atomic_operand(i: &str) -> IResult<&str, Ast> {
    let brackets_or_num = alt((bracketed_expression, map(complete::i64, |n| Ast::Value(n))));
    delimited(space0, brackets_or_num, space0)(i)
}

// Change to parse an enum
fn operator(i: &str) -> IResult<&str, Ast> {
    let symbol = alt((complete::char('+'), complete::char('*')));

    let to_op = map(symbol, |c| match c {
        '+' => Op::Add,
        '*' => Op::Multiply,
        _ => unreachable!("Not parsed"),
    });

    map(to_op, |op| Ast::Op(op))(i)
}

fn expression(i: &str) -> IResult<&str, Ast> {
    let sequence = many0(alt((atomic_operand, operator)));
    map(sequence, |list| Ast::Expression(list))(i)
}

fn solve_expr(items: Vec<Ast>, with_precedence: bool) -> i64 {
    // Start by flattening the AST, recursively resolving the sub-expressions
    let flat: Vec<Ast> = items
        .into_iter()
        .map(|elm| match elm {
            Ast::Expression(sub) => Ast::Value(solve_expr(sub, with_precedence)),
            e => e,
        })
        .collect();

    if flat.len() == 1 {
        match flat.get(0) {
            Some(Ast::Value(v)) => *v,
            _ => panic!("Bad expression syntax"),
        }
    } else if with_precedence {
        solve_flat_expr_with_precedence(flat)
    } else {
        solve_flat_expr_without_precedence(flat)
    }
}

fn solve_flat_expr_without_precedence(flat: Vec<Ast>) -> i64 {
    // Take the first operand
    let init = match flat.get(0) {
        Some(Ast::Value(v)) => *v,
        _ => panic!("First item in the expression is not a value"),
    };

    // Takes pairs of operators and operands, and apply them
    let rest = &flat[1..];
    rest.chunks(2).fold(init, |lhs, part| match &part {
        &[Ast::Op(Op::Add), Ast::Value(rhs)] => lhs + rhs,
        &[Ast::Op(Op::Multiply), Ast::Value(rhs)] => lhs * rhs,
        _ => panic!("Bad expression syntax"),
    })
}

fn solve_flat_expr_with_precedence(flat: Vec<Ast>) -> i64 {
    #[derive(Debug)]
    struct FoldState {
        operand: Option<i64>,
        operator: Option<Op>,
        result: i64,
    }

    let fold_result = flat.into_iter().fold(
        FoldState {
            operand: None,
            operator: None,
            result: 1,
        },
        |mut state, elm| {
            match (elm, state.operator, state.operand) {
                // Left-most operand in the expression
                (Ast::Value(v), None, None) => state.operand = Some(v),
                // Each operator in the expression
                (Ast::Op(o), None, Some(_)) => state.operator = Some(o),
                // We now have a full add expression
                (Ast::Value(rhs), Some(Op::Add), Some(lhs)) => {
                    state.operand = Some(lhs + rhs);
                    state.operator = None;
                }
                // We now have a full multiply expression
                (Ast::Value(rhs), Some(Op::Multiply), Some(lhs)) => {
                    state.result = state.result * lhs;
                    state.operand = Some(rhs);
                    state.operator = None;
                }
                _ => panic!("Invalid expression flow"),
            }
            state
        },
    );

    // Now perform the final multiply for the right-most operand
    match fold_result {
        FoldState {
            operator: None,
            operand: Some(last),
            result: product,
        } => last * product,
        _ => panic!("Invalid fold result"),
    }
}

fn parse_and_evaluate(expr: &str, with_precedence: bool) -> i64 {
    let (_, o) = all_consuming(expression)(expr).unwrap();
    match o {
        Ast::Expression(items) => solve_expr(items, with_precedence),
        _ => panic!("Expected an expression, but it was not"),
    }
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
        assert_eq!(71, parse_and_evaluate("1 + 2 * 3 + 4 * 5 + 6", false));
        assert_eq!(231, parse_and_evaluate("1 + 2 * 3 + 4 * 5 + 6", true));
    }

    #[test]
    fn example_2() {
        assert_eq!(51, parse_and_evaluate("1 + (2 * 3) + (4 * (5 + 6))", false));
        assert_eq!(51, parse_and_evaluate("1 + (2 * 3) + (4 * (5 + 6))", true));
    }

    #[test]
    fn example_3() {
        assert_eq!(26, parse_and_evaluate("2 * 3 + (4 * 5)", false));
        assert_eq!(46, parse_and_evaluate("2 * 3 + (4 * 5)", true));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            437,
            parse_and_evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", false)
        );
        assert_eq!(
            1445,
            parse_and_evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", true)
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            12240,
            parse_and_evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false)
        );
        assert_eq!(
            669060,
            parse_and_evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true)
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            13632,
            parse_and_evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false)
        );
        assert_eq!(
            23340,
            parse_and_evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true)
        );
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(4491283311856, calculate_part_1(&inputs));
        assert_eq!(68852578641904, calculate_part_2(&inputs));
    }
}
