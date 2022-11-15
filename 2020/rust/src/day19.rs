use std::{
    borrow::Cow,
    cell::Cell,
    collections::{HashMap, HashSet},
    fmt,
};

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
    enable_verbose: Cell<bool>,
}

impl Day19 {
    pub fn verbose_log(&self, args: fmt::Arguments) {
        if self.enable_verbose.get() {
            eprintln!("{}", args);
        }
    }
}

fn inputs() -> Day19 {
    let text = crate::util::read_file("inputs/day19.txt");
    let (_, o) = input_parser()(&text).unwrap();
    o
}

fn calculate_part_1(input: &Day19) -> usize {
    input.messages.iter().filter(|m| is_match(input, m)).count()
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
    input.messages.iter().filter(|m| is_match(input, m)).count()
}

const VERBOSE_FOR: [&'static str; 3] = [
    "bbbbbbbaaaabbbbaaabbabaaa",
    "bbbababbbbaaaaaaaabbababaaababaabab",
    "abbbbabbbbaaaababbbbbbaaaababb",
];

fn is_match(model: &Day19, message: &str) -> bool {
    let mut alternatives = AlternativesState {
        failed: HashSet::new(),
        maybe: HashSet::new(),
        stack: Vec::new(),
    };

    if VERBOSE_FOR.contains(&message) {
        model.enable_verbose.set(true);
    } else {
        model.enable_verbose.set(false);
    }

    loop {
        match evaluate_rule(0, &message, 0, model, "", &mut alternatives) {
            None => {
                eprintln!("Message {} is not a match", message);
                return false;
            }
            Some(result) => {
                if result.remaining.is_empty() {
                    eprintln!("Message {} is a match", message);
                    return true;
                }
                if result.is_exhausted {
                    eprintln!(
                        "Message {} is not a match. Leftover: {}",
                        message, result.remaining
                    );

                    return false;
                }
                eprintln!("Rule 0 was not exhausted...");
            }
        }
    }
}

struct EvaluateResult<'a> {
    remaining: &'a str,
    is_exhausted: bool,
    path_update: Option<String>,
}

#[derive(PartialEq)]
struct Candidate {
    pos: usize,
    rule_num: RuleNum,
}

struct AlternativesState {
    failed: HashSet<String>,
    maybe: HashSet<String>,
    stack: Vec<Candidate>,
}

// Recursively evaluates rules on the supplied text.
// On a successful match, it returns the remaining (unmatched) text.
// On an unsuccessful match, it returns None
fn evaluate_rule<'a>(
    pos: usize,
    text: &'a str,
    rule_num: RuleNum,
    model: &Day19,
    path: &str,
    alternatives: &mut AlternativesState,
) -> Option<EvaluateResult<'a>> {
    if text.is_empty() {
        return None;
    }

    let unwind_stack_to = alternatives.stack.len();
    let candidate = Candidate { pos, rule_num };

    if alternatives.stack.iter().rev().contains(&candidate) {
        eprintln!("Avoiding infinite recursion...");
        return None;
    } else {
        alternatives.stack.push(candidate);
    }

    let rule = model
        .rules
        .get(&rule_num)
        .expect("The rule number to exist");

    let result = match &rule.spec {
        RuleSpec::Literal(c) => {
            if text.starts_with(*c) {
                Some(EvaluateResult {
                    remaining: &text[1..],
                    is_exhausted: true,
                    path_update: None,
                })
            } else {
                None
            }
        }
        RuleSpec::Sequence(rule_nums) => {
            evaluate_sequence(pos, text, model, path, alternatives, &rule_nums)
        }
        RuleSpec::Alternation((sequence_x, sequence_y)) => evaluate_alternation(
            pos,
            text,
            model,
            path,
            alternatives,
            &sequence_x,
            &sequence_y,
        ),
    };

    while alternatives.stack.len() > unwind_stack_to {
        alternatives.stack.pop();
    }

    result
}

fn evaluate_alternation<'a>(
    pos: usize,
    text: &'a str,
    model: &Day19,
    path: &str,
    alternatives: &mut AlternativesState,
    sequence_x: &[RuleNum],
    sequence_y: &[RuleNum],
) -> Option<EvaluateResult<'a>> {
    let x_path = format!("{}X", path);
    let y_path = format!("{}Y", path);

    let x_failed = alternatives.failed.contains(&x_path);
    let y_failed = alternatives.failed.contains(&y_path);
    let x_maybe = alternatives.maybe.contains(&x_path);
    let y_maybe = alternatives.maybe.contains(&y_path);

    let (skip_x, skip_y) = match (x_failed, y_failed, x_maybe, y_maybe) {
        (false, false, false, false) => (false, false), // 0000 Nothing tried
        (false, false, false, true) => unreachable!(),  // 0001
        (false, false, true, false) => (true, false),   // 0010 X works, but Y not yet tried
        (false, false, true, true) => {
            todo!()
        }
        (false, true, false, false) => unreachable!(), // 0100
        (false, true, false, true) => unreachable!(),  // 0101
        (false, true, true, false) => (false, true),   // 0110 Only X works
        (false, true, true, true) => unreachable!(),   // 0111
        (true, false, false, false) => (true, false),  // 1000 X does not work, Y not yet tried
        (true, false, false, true) => (true, false),   // 1001 X does not work, Y does work
        (true, false, true, false) => unreachable!(),  // 1010
        (true, false, true, true) => unreachable!(),   // 1011
        (true, true, false, false) => (false, false),  // 1100 Neither work
        (true, true, false, true) => unreachable!(),   // 1101
        (true, true, true, false) => unreachable!(),   // 1110
        (true, true, true, true) => unreachable!(),    // 1111
    };

    let x_result = if skip_x {
        model.verbose_log(format_args!(
            "Returned to path: {} at position {} with remainder: {}",
            &x_path, pos, text,
        ));
        None
    } else {
        evaluate_sequence(pos, text, model, &x_path, alternatives, sequence_x)
    };

    match x_result {
        Some(result) => {
            // let path_update = if result.is_exhausted {
            //     let u = result.path_update.unwrap_or_else(|| x_path.clone());
            //     model.verbose_log(format_args!(
            //         "Excluding exhausted path: {} at position {} with remainder: {}",
            //         &x_path, pos, result.remaining,
            //     ));
            //     alternatives.maybe.insert(x_path);
            //     u
            // } else {
            //     result.path_update.unwrap_or(x_path)
            // };
            let path_update = result.path_update.unwrap_or_else(|| x_path.clone());
            if !x_maybe {
                alternatives.maybe.insert(x_path);
            }

            Some(EvaluateResult {
                remaining: result.remaining,
                is_exhausted: y_failed && result.is_exhausted,
                path_update: Some(path_update),
            })
        }
        None => {
            if !skip_x {
                alternatives.failed.insert(x_path);
            }

            if skip_y {
                model.verbose_log(format_args!(
                    "Returned to path: {} at position {} with remainder: {}",
                    &y_path, pos, text,
                ));
                return None;
            }

            let y_result = evaluate_sequence(pos, text, model, &y_path, alternatives, sequence_y);

            match y_result {
                Some(result) => {
                    // let path_update = if result.is_exhausted {
                    //     let u = result.path_update.unwrap_or_else(|| y_path.clone());
                    //     model.verbose_log(format_args!(
                    //         "Excluding exhausted path: {} at position {} with remainder: {}",
                    //         &y_path, pos, result.remaining,
                    //     ));
                    //     alternatives.maybe.insert(y_path);
                    //     u
                    // } else {
                    //     result.path_update.unwrap_or(y_path)
                    // };
                    let path_update = result.path_update.unwrap_or_else(|| y_path.clone());
                    if !y_maybe {
                        alternatives.maybe.insert(y_path);
                    }

                    Some(EvaluateResult {
                        remaining: result.remaining,
                        is_exhausted: x_failed && result.is_exhausted,
                        path_update: Some(path_update),
                    })
                }
                None => {
                    if !skip_y {
                        alternatives.failed.insert(y_path);
                    }
                    None
                }
            }
        }
    }
}

fn evaluate_sequence<'a, 'b>(
    pos: usize,
    text: &'a str,
    model: &Day19,
    path: &'b str,
    alternatives: &mut AlternativesState,
    rule_number_sequence: &[RuleNum],
) -> Option<EvaluateResult<'a>> {
    #[derive(Clone)]
    struct FoldState<'a, 'b> {
        text: &'a str,
        pos: usize,
        idx: usize,
        path: Cow<'b, str>,
    }

    let mut state = FoldState {
        text,
        pos,
        idx: 0,
        path: Cow::Borrowed(path),
    };
    let mut backtracks: Vec<FoldState> = Vec::new();

    'rules: loop {
        let rule_num = rule_number_sequence[state.idx];

        let sub_result = evaluate_rule(
            state.pos,
            state.text,
            rule_num,
            model,
            &state.path,
            alternatives,
        );

        if let None = sub_result {
            if let Some(pop_state) = backtracks.pop() {
                state = pop_state;
                continue 'rules;
            } else {
                return None;
            }
        }

        let sub_result = sub_result.unwrap();

        if !sub_result.is_exhausted {
            backtracks.push(state.clone());
        }

        // The sub rule successfully matched with an alternation, so update our path
        if let Some(path) = sub_result.path_update {
            state.path = Cow::Owned(path);
        }

        let advanced_by = state.text.len() - sub_result.remaining.len();

        state.pos += advanced_by;
        state.text = sub_result.remaining;
        state.idx += 1;

        // We've finished matching the sequence successfully
        if state.idx == rule_number_sequence.len() {
            break;
        }
    }

    let path_update = match state.path {
        Cow::Borrowed(b) if b == path => None,
        Cow::Borrowed(b) => Some(b.to_owned()), // Paranoia, but shouldn't ever happen
        Cow::Owned(o) => Some(o),
    };

    Some(EvaluateResult {
        remaining: state.text,
        is_exhausted: backtracks.is_empty(),
        path_update: path_update,
    })
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
            enable_verbose: Cell::new(false),
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
