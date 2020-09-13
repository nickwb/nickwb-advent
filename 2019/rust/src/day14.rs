use std::collections::{HashMap, VecDeque};

use regex::Regex;

const ORE_IDX: usize = 0;
const FUEL_IDX: usize = 1;

#[derive(Debug)]
struct InputInterpretation {
    compount_count: usize,
    compound_map: HashMap<String, usize>,
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

                cost[compound_idx] = quantity;
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
        for r in self.reactions.iter_mut() {
            r.cost.resize(self.compount_count, 0);
        }
    }
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

#[derive(Clone)]
struct SearchCandidate {
    demand_queue: VecDeque<Output>,
    overflow: Vec<usize>,
    total_ore: usize,
}

enum DemandType<'a> {
    NoFurtherDemands,
    NeedsOutput(&'a Output),
}

enum ApplyResult {
    NewCandidate(SearchCandidate),
    AllDemandsMet(SearchCandidate),
}

impl SearchCandidate {
    fn initial(input: &InputInterpretation) -> SearchCandidate {
        let mut demand_queue = VecDeque::with_capacity(input.compount_count);
        demand_queue.push_back(Output {
            compound_idx: FUEL_IDX,
            quantity: 1,
        });
        let mut overflow = Vec::with_capacity(input.compount_count);
        overflow.resize(input.compount_count, 0);

        SearchCandidate {
            demand_queue,
            overflow,
            total_ore: 0,
        }
    }

    fn get_next_demand(&self) -> DemandType<'_> {
        match self.demand_queue.get(0) {
            Some(d) => DemandType::NeedsOutput(d),
            None => DemandType::NoFurtherDemands,
        }
    }

    fn update_next_demand(&mut self, new_value: Output) {
        let head = self
            .demand_queue
            .get_mut(0)
            .expect("Tried to update the next demand, but there isn't one");

        head.compound_idx = new_value.compound_idx;
        head.quantity = new_value.quantity;
    }

    fn consume_overflow(&mut self) {
        loop {
            let lesser_demand = {
                let demand = match self.get_next_demand() {
                    DemandType::NeedsOutput(d) => d,
                    DemandType::NoFurtherDemands => {
                        return;
                    }
                };
                let compound = demand.compound_idx;

                // No overflow to consume for the next demand
                if self.overflow[compound] == 0 {
                    return;
                }

                // More overflow than demand.
                // So, this demand is already satisfied.
                if self.overflow[compound] >= demand.quantity {
                    self.overflow[compound] -= demand.quantity;
                    self.demand_queue.pop_front();
                    continue;
                }

                // Demand is higher than the available overflow,
                // so we can adjust the demand down and discard the overflow
                let lesser_demand = Output {
                    quantity: demand.quantity - self.overflow[compound],
                    compound_idx: compound,
                };
                self.overflow[compound] = 0;

                lesser_demand
            };

            self.update_next_demand(lesser_demand);
            return;
        }
    }

    fn apply_reaction(&self, reaction: &Reaction) -> ApplyResult {
        let demand = match self.get_next_demand() {
            DemandType::NeedsOutput(d) => d,
            DemandType::NoFurtherDemands => panic!("Expected an unmet demand"),
        };

        let compound = demand.compound_idx;

        if reaction.output.compound_idx != compound {
            panic!("Forming the wrong compound.");
        }

        let mut new_candidate = self.clone();

        if reaction.output.quantity >= demand.quantity {
            new_candidate.demand_queue.pop_front();
            new_candidate.overflow[compound] += reaction.output.quantity - demand.quantity;
        } else {
            new_candidate.demand_queue[0].quantity -= reaction.output.quantity;
        }

        for (i, &cost) in reaction.cost.iter().enumerate() {
            if cost > 0 {
                match i {
                    ORE_IDX => new_candidate.total_ore += cost,
                    FUEL_IDX => panic!("This looks recursive..."),
                    _ => new_candidate.demand_queue.push_back(Output {
                        compound_idx: i,
                        quantity: cost,
                    }),
                }
            }
        }

        if new_candidate.demand_queue.is_empty() {
            ApplyResult::AllDemandsMet(new_candidate)
        } else {
            ApplyResult::NewCandidate(new_candidate)
        }
    }
}

fn find_min_ore(input: &InputInterpretation) -> usize {
    let mut candidates: VecDeque<SearchCandidate> = VecDeque::new();
    candidates.push_back(SearchCandidate::initial(input));

    while !candidates.is_empty() {
        let mut prior = candidates.pop_front().unwrap();
        prior.consume_overflow();

        let demand = match prior.get_next_demand() {
            DemandType::NeedsOutput(d) => d,
            DemandType::NoFurtherDemands => return prior.total_ore,
        };
        let compound = demand.compound_idx;

        for r in input.reactions.iter() {
            if r.output.compound_idx == compound {
                match prior.apply_reaction(r) {
                    ApplyResult::AllDemandsMet(found_result) => {
                        return found_result.total_ore;
                    }
                    ApplyResult::NewCandidate(next) => {
                        candidates.push_back(next);
                    }
                }
            }
        }
    }

    panic!("Reached end of candidates without forming fuel");
}

fn inputs() -> InputInterpretation {
    let text = crate::util::read_file("inputs/day14.txt");
    InputInterpretation::parse_input(&text).unwrap()
}

pub fn run_day_fourteen() {
    let input = inputs();
    println!("Day 14, Part 1: {}", find_min_ore(&input));
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

#[test]
fn example_3() {
    let text = r"
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    ";

    let input = InputInterpretation::parse_input(text).unwrap();
    assert_eq!(13312, find_min_ore(&input));
}

#[test]
fn example_4() {
    let text = r"
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    ";

    let input = InputInterpretation::parse_input(text).unwrap();
    assert_eq!(180697, find_min_ore(&input));
}

#[test]
fn example_5() {
    let text = r"
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    ";

    let input = InputInterpretation::parse_input(text).unwrap();
    assert_eq!(2210736, find_min_ore(&input));
}

#[test]
fn actual_part_1() {
    let input = inputs();
    assert_eq!(431448, find_min_ore(&input));
}
