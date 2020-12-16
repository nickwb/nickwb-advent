use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn run_day_sixteen() {
    let inputs = inputs();
    println!("Day 16, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 16, Part 2: {}", calculate_part_2(&inputs));
}

fn calculate_part_1(input: &Day16Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .flat_map(|t| t.iter())
        .filter(|&&n| !is_valid_for_any_rule(input, n))
        .sum()
}

fn is_valid_for_any_rule(input: &Day16Input, n: usize) -> bool {
    input
        .rules
        .iter()
        .any(|rule| rule.ranges.iter().any(|r| n >= r.0 && n <= r.1))
}

fn find_single_matching_rule<'a>(
    rules: &[&'a FieldRule],
    nums: &[usize],
) -> Option<(usize, &'a FieldRule)> {
    rules
        .iter()
        .enumerate()
        .filter_map(|(idx, &rule)| {
            if nums
                .iter()
                .all(|&n| rule.ranges.iter().any(|r| n >= r.0 && n <= r.1))
            {
                Some((idx, rule))
            } else {
                None
            }
        })
        .exactly_one()
        .ok()
}

fn calculate_part_2(input: &Day16Input) -> usize {
    let mut groups: HashMap<usize, Vec<usize>> = input
        .nearby_tickets
        .iter()
        // Discard invalid nearby tickets
        .filter(|t| t.iter().all(|&n| is_valid_for_any_rule(input, n)))
        // Split tickets in to (idx, num) pairs
        .flat_map(|t| t.iter().enumerate().map(|(i, n)| (i, *n)))
        // My ticket must also match the rules
        .chain(input.my_ticket.iter().enumerate().map(|(i, n)| (i, *n)))
        // Group all of the same indexes together
        .into_group_map();

    let mut unmatched_rules: Vec<&FieldRule> = input.rules.iter().collect();
    let mut matched_rules: HashMap<usize, &FieldRule> = HashMap::new();

    while !groups.is_empty() {
        // Find a group which can only match a single rule
        let matched = groups
            .iter()
            .find_map(|(&i, nums)| Some((i, find_single_matching_rule(&unmatched_rules, nums)?)))
            .expect("At least one matching rule");

        // Gogo confusing nested tuple
        let num_idx = matched.0;
        let unmatched_rule_idx = matched.1 .0;
        let rule = matched.1 .1;

        // Don't need to match this group any more
        groups.remove(&num_idx);

        // Don't need to match this rule anymore
        unmatched_rules.remove(unmatched_rule_idx);

        matched_rules.insert(num_idx, rule);
    }

    input
        .my_ticket
        .iter()
        .enumerate()
        .filter_map(|(i, &n)| {
            let rule = matched_rules.get(&i).expect("There should be a rule");
            if rule.name.starts_with("departure") {
                Some(n)
            } else {
                None
            }
        })
        .product()
}

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Day16Input {
    rules: Vec<FieldRule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

type ValidRange = (usize, usize);

#[derive(Debug)]
struct FieldRule {
    name: String,
    ranges: Vec<ValidRange>,
}

fn inputs() -> Day16Input {
    let text = crate::util::read_file("inputs/day16.txt");
    parse(&text)
}

enum ParsePhase {
    FieldRules,
    YourTicket,
    NearbyTickets,
}

lazy_static! {
    static ref RULE_PATTERN: Regex = Regex::new(r"^([a-z\s]+): ([\d\-]+) or ([\d\-]+)$").unwrap();
}

fn parse(text: &str) -> Day16Input {
    let input = Day16Input {
        rules: Vec::new(),
        my_ticket: Vec::new(),
        nearby_tickets: Vec::new(),
    };
    text.lines()
        .filter_map(crate::util::not_blank)
        .fold(
            (ParsePhase::FieldRules, input),
            |(phase, mut input), line| match (phase, line) {
                (_, "your ticket:") => (ParsePhase::YourTicket, input),
                (_, "nearby tickets:") => (ParsePhase::NearbyTickets, input),
                (ParsePhase::YourTicket, x) => {
                    input.my_ticket = parse_ticket(x);
                    (ParsePhase::YourTicket, input)
                }
                (ParsePhase::NearbyTickets, x) => {
                    input.nearby_tickets.push(parse_ticket(x));
                    (ParsePhase::NearbyTickets, input)
                }
                (ParsePhase::FieldRules, x) => {
                    let captures = RULE_PATTERN.captures(x).expect("Valid rule");
                    let rule = FieldRule {
                        name: captures[1].to_owned(),
                        ranges: vec![parse_range(&captures[2]), parse_range(&captures[3])],
                    };
                    input.rules.push(rule);
                    (ParsePhase::FieldRules, input)
                }
            },
        )
        .1
}

fn parse_ticket(text: &str) -> Ticket {
    text.split(',')
        .map(|n| n.parse::<usize>().expect("Valid ticket digits"))
        .collect()
}

fn parse_range(text: &str) -> ValidRange {
    let mut parts = text.split('-');
    let min = parts
        .next()
        .expect("Min Bound")
        .parse::<usize>()
        .expect("Valid min bound");
    let max = parts
        .next()
        .expect("Max Bound")
        .parse::<usize>()
        .expect("Valid max bound");
    assert_eq!(None, parts.next());

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12
        ";

        assert_eq!(71, calculate_part_1(&parse(text)));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(27802, calculate_part_1(&inputs));
        assert_eq!(279139880759, calculate_part_2(&inputs));
    }
}
