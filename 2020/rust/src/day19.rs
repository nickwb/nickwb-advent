use std::collections::HashMap;

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

fn is_match(model: &Day19, message: &str) -> bool {
    let state = SearchState {
        input_model: model,
        _message: message,
    };
    let candidate = Candidate {
        choice_depth: 0,
        choices: 0,
        consumed: 0,
        remaining: message,
        stack: Vec::new(),
    };
    search(&state, candidate, 0)
}

struct SearchState<'im> {
    input_model: &'im Day19,
    _message: &'im str,
}

#[derive(Clone, Copy)]
enum Bookmark {
    Sequence {
        rule: RuleNum,
        idx: usize,
    },
    AltSequence {
        rule: RuleNum,
        branch: u8,
        idx: usize,
    },
}

/// Represents a partially matched message
#[derive(Clone)]
struct Candidate<'im> {
    remaining: &'im str,
    stack: Vec<Bookmark>,
    // For debugging: How much of the message have we parsed already
    consumed: u16,
    // For debugging: A bitstring of all of the branches we have taken
    choices: u128,
    // For debugging: The number of branches we have taken so far
    choice_depth: u8,
}

impl<'im> Candidate<'im> {
    pub fn next_char(&self) -> Option<char> {
        self.remaining.chars().next()
    }

    pub fn advance_one(&mut self) {
        self.consumed += 1;
        self.remaining = &self.remaining[1..];
    }

    pub fn make_choice(&mut self, x_branch: bool) {
        if !x_branch {
            let mask = 1 << (self.choice_depth as u128);
            self.choices = self.choices & mask;
        }
        self.choice_depth += 1;
    }
}

// This is a recursive search, which tries to match the whole of the remaining message.
// This method starts at a given rule number, and continues matching until either the whole message
// has been matched, or we conclude that a message does not match.
// Alternations are tricky. Locally, it may be true that BOTH branches can be successfully matched,
// but this may not be true globally: as different branches may consume different amounts of input.
// This is why this recursion matches the whole of the remaining input.
// Arising from this construction is the bookmark system: our way of notating rule sequences which
// have only been partially evaluated. Once we reach a leaf rule (a literal), we continue our
// recursion deeper by calling resume, which will pop bookmarks off the stack and continue the
// search until we have finished matching every rule and every character in the message.
fn search<'im>(state: &'im SearchState<'im>, mut candidate: Candidate<'im>, rule: RuleNum) -> bool {
    let spec = &state.input_model.rules[&rule].spec;

    match spec {
        RuleSpec::Literal(c) => {
            if *c == candidate.next_char().expect("More input to consume") {
                candidate.advance_one();

                // This character is a match, but we must now continue searching
                // through partially evaluated rules to confirm the whole match
                resume(state, candidate)
            } else {
                // This candidate does not match
                false
            }
        }
        RuleSpec::Sequence(rules) => {
            if rules.len() > 1 {
                // There's more than one rule in this sequence, so we'll need to return to the
                // subsequent rules after the first index is evaluated
                candidate.stack.push(Bookmark::Sequence { rule, idx: 1 });
            }
            search(state, candidate, rules[0])
        }
        RuleSpec::Alternation((rules_x, rules_y)) => {
            let mut candidate_x = candidate.clone();
            candidate_x.make_choice(true);

            if rules_x.len() > 1 {
                // There's more than one rule in this sequence, so we'll need to return to the
                // subsequent rules after the first index is evaluated
                candidate_x.stack.push(Bookmark::AltSequence {
                    rule,
                    branch: 0,
                    idx: 1,
                });
            }
            let result_x = search(state, candidate_x, rules_x[0]);

            if result_x {
                // The first branch was a complete match, so we don't need to search the other branch
                true
            } else {
                let mut candidate_y = candidate;
                candidate_y.make_choice(false);

                if rules_y.len() > 1 {
                    // There's more than one rule in this sequence, so we'll need to return to the
                    // subsequent rules after the first index is evaluated
                    candidate_y.stack.push(Bookmark::AltSequence {
                        rule,
                        branch: 1,
                        idx: 1,
                    });
                }
                search(state, candidate_y, rules_y[0])
            }
        }
    }
}

// Continue searching for a match on the given candidate, resolving any suspended rules on the stack
fn resume<'im>(state: &'im SearchState<'im>, mut candidate: Candidate<'im>) -> bool {
    let has_more_input = !candidate.remaining.is_empty();
    let has_more_rules = !candidate.stack.is_empty();

    // First check if we are either fully matched, or we have reached a failure state
    match (has_more_input, has_more_rules) {
        (false, false) => return true,
        (true, false) | (false, true) => return false,
        (true, true) => {}
    }

    // We didn't finish evaluating this rule:
    let resume_to = candidate.stack.pop().expect("More rules to match");

    match resume_to {
        Bookmark::Sequence { rule, idx } => {
            if let RuleSpec::Sequence(seq) = &state.input_model.rules[&rule].spec {
                let is_last = idx + 1 == seq.len();
                if !is_last {
                    // Still not at the end
                    candidate
                        .stack
                        .push(Bookmark::Sequence { rule, idx: idx + 1 });
                }
                search(state, candidate, seq[idx])
            } else {
                panic!("Sequence bookmark wasn't a sequence")
            }
        }
        Bookmark::AltSequence { rule, branch, idx } => {
            if let RuleSpec::Alternation((seq_x, seq_y)) = &state.input_model.rules[&rule].spec {
                let seq = if branch == 0 { seq_x } else { seq_y };
                let is_last = idx + 1 == seq.len();
                if !is_last {
                    // Still not at the end
                    candidate.stack.push(Bookmark::AltSequence {
                        rule,
                        branch,
                        idx: idx + 1,
                    });
                }
                search(state, candidate, seq[idx])
            } else {
                panic!("AltSequence bookmark wasn't an Alternation")
            }
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
        let mut inputs = inputs();
        assert_eq!(113, calculate_part_1(&inputs));
        assert_eq!(253, calculate_part_2(&mut inputs));
    }
}
