use std::collections::{HashMap, VecDeque};

use regex::Regex;

const ORE_IDX: usize = 0;
const FUEL_IDX: usize = 1;

#[derive(Debug)]
struct InputInterpretation {
    compount_count: usize,
    compound_map: HashMap<String, usize>,
    max_cost: Vec<usize>,
    reactions: Vec<Reaction>,
}

#[derive(Debug, Clone)]
struct Reaction {
    pub cost: Vec<usize>,
    pub output: Output,
    pub idx: usize,
}

#[derive(Debug, Clone)]
struct Output {
    pub compound_idx: usize,
    pub quantity: usize,
}

impl InputInterpretation {
    fn new() -> InputInterpretation {
        let mut result = InputInterpretation {
            compount_count: 2,
            compound_map: HashMap::new(),
            max_cost: Vec::new(),
            reactions: Vec::new(),
        };

        result.compound_map.insert("ORE".to_owned(), ORE_IDX);
        result.compound_map.insert("FUEL".to_owned(), FUEL_IDX);

        result
    }

    fn learn_compound(&mut self, name: String) -> usize {
        if self.compound_map.contains_key(&name) {
            return self.compound_map[&name];
        }

        let idx = self.compount_count;
        self.compount_count += 1;

        self.compound_map.insert(name, idx);
        idx
    }

    fn parse_input(text: &str) -> Option<InputInterpretation> {
        let mut result = InputInterpretation::new();

        for line in text.lines() {
            if !line.contains("=>") {
                continue;
            }

            let mut inputs_and_outputs = line.split("=>");
            let inputs = inputs_and_outputs.next().expect("No inputs to reaction");
            let outputs = inputs_and_outputs.next().expect("No outputs to reaction");
            if let Some(_) = inputs_and_outputs.next() {
                panic!("Invalid reaction.");
            }

            let mut cost: Vec<usize> = Vec::with_capacity(result.compount_count);

            for ingredient in inputs.split(",") {
                let (quantity, compound) =
                    parse_quantity_and_compound(ingredient).expect("Invalid input ingredient");
                let compound_idx = result.learn_compound(compound);

                while cost.len() <= compound_idx {
                    cost.push(0);
                }

                while result.max_cost.len() <= compound_idx {
                    result.max_cost.push(0);
                }

                cost[compound_idx] = quantity;
                result.max_cost[compound_idx] = result.max_cost[compound_idx].max(quantity);
            }

            let (output_quantity, output_compound) =
                parse_quantity_and_compound(outputs).expect("Invalid reaction output");

            let output_compound_idx = result.learn_compound(output_compound);

            let reaction = Reaction {
                cost: cost,
                output: Output {
                    compound_idx: output_compound_idx,
                    quantity: output_quantity,
                },
                idx: result.reactions.len(),
            };

            result.reactions.push(reaction);
        }

        result.normalise_costs();

        Some(result)
    }

    fn normalise_costs(&mut self) {
        self.max_cost.resize(self.compount_count, 0);

        for r in self.reactions.iter_mut() {
            r.cost.resize(self.compount_count, 0);
        }
    }
}

#[derive(Clone)]
struct SearchCandidate {
    quantities_on_hand: Vec<usize>,
    total_ore: usize,
}

impl SearchCandidate {
    fn initial(input: &InputInterpretation) -> SearchCandidate {
        let mut quantities = Vec::with_capacity(input.compount_count);
        quantities.resize(input.compount_count, 0);
        SearchCandidate {
            quantities_on_hand: quantities,
            total_ore: 0,
        }
    }
}

fn find_min_ore(input: &InputInterpretation) -> usize {
    // let sorted_recipies = input.reactions.clone();
    // sorted_recipies.sort_by(|a, b| {

    // })

    let mut candidates: VecDeque<SearchCandidate> = VecDeque::new();
    candidates.push_back(SearchCandidate::initial(input));

    while !candidates.is_empty() {
        let prior = candidates.pop_front().unwrap();

        for r in input.reactions.iter() {
            let (_, can_afford) = r.cost.iter().fold(
                (0, true),
                |(idx, can_afford): (usize, bool), &cost: &usize| {
                    (
                        idx + 1,
                        can_afford && (idx == ORE_IDX || cost <= prior.quantities_on_hand[idx]),
                    )
                },
            );

            if !can_afford {
                continue;
            }

            let worth_pursuing = r.output.compound_idx == FUEL_IDX
                || input.max_cost[r.output.compound_idx]
                    > prior.quantities_on_hand[r.output.compound_idx];

            if !worth_pursuing {
                continue;
            }

            let mut next_candidate = prior.clone();
            next_candidate.total_ore += r.cost[ORE_IDX];
            for i in 0..input.compount_count {
                if i != ORE_IDX {
                    next_candidate.quantities_on_hand[i] -= r.cost[i];
                }
            }
            next_candidate.quantities_on_hand[r.output.compound_idx] += r.output.quantity;

            if r.output.compound_idx == FUEL_IDX && r.output.quantity > 0 {
                return next_candidate.total_ore;
            }

            candidates.push_back(next_candidate);
        }
    }

    panic!("Reached end of candidates without finding fuel");
}

lazy_static! {
    static ref QUANTITY_AND_COMPOUND_PATTERN: Regex = Regex::new(r"^(\d+) *([A-Z]+)$").unwrap();
}

fn parse_quantity_and_compound(text: &str) -> Option<(usize, String)> {
    let captures = QUANTITY_AND_COMPOUND_PATTERN.captures(text.trim())?;
    let quantity = captures.get(1)?.as_str().parse::<usize>().ok()?;
    let compound = captures.get(2)?.as_str().to_owned();
    Some((quantity, compound))
}

#[test]
fn example_1() {
    let text = r"
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ";

    let input = InputInterpretation::parse_input(text).unwrap();
    assert_eq!(31, find_min_ore(&input));
}

#[test]
fn example_2() {
    let text = r"
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ";

    let input = InputInterpretation::parse_input(text).unwrap();
    assert_eq!(165, find_min_ore(&input));
}
