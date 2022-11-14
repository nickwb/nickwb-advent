use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, alpha1, line_ending, multispace0, satisfy, space0},
    combinator::{all_consuming, map},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult, Parser,
};

pub fn run_day_nineteen() {
    let mut inputs = inputs();
    println!("Day 19, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 19, Part 2: {}", calculate_part_2(&mut inputs));
}

type RuleNum = i32;

#[derive(Debug)]
enum RuleSpec {
    Literal(char),
    Sequence(Vec<RuleNum>),
    Alternation((Vec<RuleNum>, Vec<RuleNum>)),
}

#[derive(Debug)]
struct MessageRule {
    num: RuleNum,
    spec: RuleSpec,
}

#[derive(Debug)]
struct Day19 {
    rules: HashMap<RuleNum, MessageRule>,
    messages: Vec<String>,
}

fn inputs() -> Day19 {
    let text = crate::util::read_file("inputs/day19.txt");
    let (_, o) = input_parser()(&text).unwrap();
    o
}

fn calculate_part_1(input: &Day19) -> usize {
    input
        .messages
        .iter()
        .filter(|m| match evaluate_rule(0, &m, 0, input, &mut Vec::new()) {
            Some(remaining) => remaining.len() == 0,
            None => false,
        })
        .count()
}

fn calculate_part_2(input: &mut Day19) -> usize {
    input.rules.insert(
        8,
        MessageRule {
            num: 8,
            spec: RuleSpec::Alternation((vec![42], vec![42, 8])),
        },
    );
    input.rules.insert(
        11,
        MessageRule {
            num: 11,
            spec: RuleSpec::Alternation((vec![42, 31], vec![42, 11, 31])),
        },
    );
    input
        .messages
        .iter()
        .filter(|m| match evaluate_rule(0, &m, 0, input, &mut Vec::new()) {
            Some(remaining) => remaining.len() == 0,
            None => false,
        })
        .count()
}

type SeenSet = Vec<(usize, RuleNum)>;

// Recursively evaluates rules on the supplied text.
// On a successful match, it returns the remaining (unmatched) text.
// On an unsuccessful match, it returns None
fn evaluate_rule<'a>(
    pos: usize,
    text: &'a str,
    idx: RuleNum,
    input: &Day19,
    seen: &mut SeenSet,
) -> Option<&'a str> {
    // If we have been at this position and this rule number before, we will recurse infinitely
    if seen.iter().rev().contains(&(pos, idx)) {
        return None;
    }

    seen.push((pos, idx));
    let backtrack_to = seen.len();

    fn backtrack(seen: &mut SeenSet, to: usize) {
        while seen.len() > to {
            seen.pop();
        }
    };

    let rule = input.rules.get(&idx)?;

    fn fold_sequence<'a>(
        pos: usize,
        text: &'a str,
        input: &Day19,
        nums: &[RuleNum],
        seen: &mut SeenSet,
    ) -> Option<&'a str> {
        let mut text = text;
        let mut pos = pos;
        for num in nums {
            let next = evaluate_rule(pos, text, *num, input, seen)?;
            pos += text.len() - next.len();
            text = next;
        }
        Some(text)
    }

    let result = match &rule.spec {
        RuleSpec::Literal(c) => {
            if text.starts_with(*c) {
                Some(&text[1..])
            } else {
                None
            }
        }
        RuleSpec::Sequence(nums) => fold_sequence(pos, text, input, &nums, seen),
        RuleSpec::Alternation((nums_a, nums_b)) => {
            let first = fold_sequence(pos, text, input, &nums_a, seen);
            match first {
                result @ Some(_) => result,
                None => {
                    backtrack(seen, backtrack_to);
                    fold_sequence(pos, text, input, &nums_b, seen)
                }
            }
        }
    };

    match result {
        good @ Some(_) => good,
        None => {
            backtrack(seen, backtrack_to);
            None
        }
    }
}

fn input_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Day19> {
    fn consume_spaces<'a, F: 'a, O, E: ParseError<&'a str>>(
        inner: F,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
    {
        delimited(space0, inner, space0)
    }

    fn build_number_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, RuleNum> {
        consume_spaces(complete::i32)
    }

    fn build_numbers_parser<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<RuleNum>> {
        many1(build_number_parser())
    }

    let parse_sequence = map(build_numbers_parser(), |vec| RuleSpec::Sequence(vec));

    let parse_literal = consume_spaces(
        delimited(
            complete::char('"'),
            satisfy(|c| c.is_alphabetic()),
            complete::char('"'),
        )
        .map(|c| RuleSpec::Literal(c)),
    );

    let parse_alternation = tuple((
        build_numbers_parser(),
        complete::char('|'),
        build_numbers_parser(),
    ))
    .map(|(a, _, b)| RuleSpec::Alternation((a, b)));

    let parse_rule_spec = consume_spaces(alt((parse_literal, parse_alternation, parse_sequence)));

    let parse_rule = tuple((build_number_parser(), complete::char(':'), parse_rule_spec))
        .map(|(num, _, spec)| MessageRule { num, spec });

    let parse_all_rules = separated_list1(line_ending, consume_spaces(parse_rule));

    let parse_message = consume_spaces(alpha1).map(|m: &str| m.to_owned());
    let parse_all_messages = separated_list1(line_ending, parse_message);

    let parse_blank_line = tuple((line_ending, space0, line_ending));

    let parse_all_inputs =
        tuple((parse_all_rules, parse_blank_line, parse_all_messages)).map(|(r, _, m)| Day19 {
            rules: r.into_iter().map(|r| (r.num, r)).collect(),
            messages: m,
        });

    all_consuming(delimited(multispace0, parse_all_inputs, multispace0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb
        "#;

        let (_, i) = input_parser()(text).unwrap();
        assert_eq!(2, calculate_part_1(&i));
    }

    #[test]
    fn example_2() {
        let text = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1
            
            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;

        let (_, mut i) = input_parser()(text).unwrap();
        assert_eq!(12, calculate_part_2(&mut i));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(113, calculate_part_1(&inputs));
        // assert_eq!(68852578641904, calculate_part_2(&inputs));
    }
}
