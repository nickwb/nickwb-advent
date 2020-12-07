use std::{collections::HashMap, collections::HashSet};

use regex::Regex;

pub fn run_day_seven() {
    let inputs = inputs();
    println!("Day 7, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 7, Part 2: {}", calculate_part_2(&inputs));
}

fn inputs() -> InputInterpretation {
    let text = crate::util::read_file("inputs/day7.txt");
    parse_input(&text)
}

const SHINY_GOLD_BAG: BagId = BagId(0);

fn calculate_part_1(input: &InputInterpretation) -> usize {
    let mut found = HashSet::new();
    get_containers_recursive(input, SHINY_GOLD_BAG, &mut found);

    found.len()
}

fn calculate_part_2(input: &InputInterpretation) -> usize {
    count_children_recursive(input, SHINY_GOLD_BAG)
}

fn get_containers_recursive(
    input: &InputInterpretation,
    captured_bag: BagId,
    found_containers: &mut HashSet<BagId>,
) {
    for r in input
        .rules
        .iter()
        .filter(|r| r.contents.iter().any(|c| c.0 == captured_bag))
    {
        found_containers.insert(r.container_bag);
        get_containers_recursive(input, r.container_bag, found_containers);
    }
}

fn count_children_recursive(input: &InputInterpretation, container_bag: BagId) -> usize {
    input
        .rules
        .iter()
        .find(|r| r.container_bag == container_bag)
        .unwrap()
        .contents
        .iter()
        .map(|c| c.1 + (c.1 * count_children_recursive(input, c.0)))
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct BagId(usize);

#[derive(Debug)]
struct InputInterpretation {
    bag_types: HashMap<String, BagId>,
    rules: Vec<BagRule>,
}

#[derive(Debug)]
struct BagRule {
    container_bag: BagId,
    contents: Vec<(BagId, usize)>,
}

lazy_static! {
    static ref BAG_PATTERN: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
}

fn parse_input(text: &str) -> InputInterpretation {
    let mut next_bag_id: usize = 1;
    let mut bag_types: HashMap<String, BagId> = HashMap::new();
    bag_types.insert("shiny gold".to_owned(), SHINY_GOLD_BAG);

    let mut rules: Vec<BagRule> = Vec::new();

    let mut get_id = |name: &str| match bag_types.get(name) {
        Some(id) => id.clone(),
        None => {
            let id = BagId(next_bag_id);
            bag_types.insert(name.to_owned(), id.clone());
            next_bag_id += 1;
            id
        }
    };

    for l in text.lines() {
        let l = l.trim();

        if l.is_empty() {
            continue;
        }

        let mut parts = l.split(" bags contain ");
        let container_bag = get_id(parts.next().expect("Expected a container"));
        let contents = parts.next().expect("Expected some contents");
        assert_eq!(None, parts.next());

        if contents.starts_with("no other bags") {
            rules.push(BagRule {
                container_bag,
                contents: Vec::new(),
            });
            continue;
        }

        let contents: Vec<(BagId, usize)> = contents
            .split(",")
            .map(|b| {
                let captures = BAG_PATTERN
                    .captures(b)
                    .expect("Expected a valid bag descriptor");
                let quantity = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let id = get_id(captures.get(2).unwrap().as_str());
                (id, quantity)
            })
            .collect();

        rules.push(BagRule {
            container_bag,
            contents,
        });
    }

    InputInterpretation { bag_types, rules }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        ";

        let input = parse_input(text);
        assert_eq!(4, calculate_part_1(&input));
        assert_eq!(32, calculate_part_2(&input));
    }

    #[test]
    fn example_2() {
        let text = r"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        ";

        let input = parse_input(text);
        assert_eq!(126, calculate_part_2(&input));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(205, calculate_part_1(&inputs));
        assert_eq!(80902, calculate_part_2(&inputs));
    }
}
