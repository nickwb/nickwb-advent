use regex::Regex;
use std::collections::{HashMap, VecDeque};

const ORE_IDX: usize = 0;
const FUEL_IDX: usize = 1;
const PART_TWO_ORE: usize = 1_000_000_000_000;

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
            };

            result.reactions.push(reaction);
        }

        result.convert_to_lut();
        result.normalise_costs();

        Some(result)
    }

    // Update the reactions Vec so that it can behave like a lookup table.
    // In this configuration, reactions[x] will always produce an output
    // of compound type x.
    fn convert_to_lut(&mut self) {
        let mut result = Vec::new();

        for i in 0..self.compount_count {
            let found = self.reactions.iter().enumerate().find_map(|(j, reaction)| {
                if reaction.output.compound_idx == i {
                    Some(j)
                } else {
                    None
                }
            });

            let reaction = match (i, found) {
                (_, Some(idx)) => self.reactions.swap_remove(idx),

                // We're never going to look this up, but let's keep rust
                // happy by guaranteeing that all elements of the reaction
                // Vec are a valid instance of the Reaction struct.
                (ORE_IDX, None) => Reaction {
                    output: Output {
                        compound_idx: ORE_IDX,
                        quantity: 1,
                    },
                    cost: Vec::new(),
                },

                (j, None) => panic!("Could not find reaction for output: {}", j),
            };

            result.push(reaction);
        }

        self.reactions = result;
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
struct SearchState {
    demand_queue: VecDeque<Output>,
    overflow: Vec<usize>,
    total_cost: Vec<usize>,
}

enum DemandType<'a> {
    NoFurtherDemands,
    NeedsOutput(&'a Output),
}

enum ApplyResult {
    ContinueSearching,
    AllDemandsMet,
}

impl SearchState {
    fn initial(input: &InputInterpretation) -> SearchState {
        let mut demand_queue = VecDeque::with_capacity(input.compount_count);
        demand_queue.push_back(Output {
            compound_idx: FUEL_IDX,
            quantity: 1,
        });

        let mut zero_resources = Vec::with_capacity(input.compount_count);
        zero_resources.resize(input.compount_count, 0);

        SearchState {
            demand_queue,
            overflow: zero_resources.clone(),
            total_cost: zero_resources,
        }
    }

    fn get_next_demand(&self) -> DemandType<'_> {
        match self.demand_queue.get(0) {
            Some(d) => DemandType::NeedsOutput(d),
            None => DemandType::NoFurtherDemands,
        }
    }

    fn ore_cost(&self) -> usize {
        self.total_cost[ORE_IDX]
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

    fn add_demand(&mut self, compound: usize, quantity: usize) {
        let existing = self
            .demand_queue
            .iter_mut()
            .find(|d| d.compound_idx == compound);

        match existing {
            Some(e) => e.quantity += quantity,
            None => self.demand_queue.push_back(Output {
                compound_idx: compound,
                quantity: quantity,
            }),
        }
    }

    fn apply_reaction(&mut self, reaction: &Reaction) -> ApplyResult {
        let demand = match self.get_next_demand() {
            DemandType::NeedsOutput(d) => d,
            DemandType::NoFurtherDemands => panic!("Expected an unmet demand"),
        };

        let compound = demand.compound_idx;

        if reaction.output.compound_idx != compound {
            panic!("Forming the wrong compound.");
        }

        // Divide demand.quantity by reaction.output.quantity, rounding up
        let multiple = (demand.quantity + reaction.output.quantity - 1) / reaction.output.quantity;
        let output_quantity = reaction.output.quantity * multiple;

        if output_quantity < demand.quantity {
            panic!("Expected to meet demand.");
        }

        // Demand borrows immutably from self, but we need to mutate ourselves
        // in the next step.
        drop(demand);

        self.overflow[compound] += output_quantity - demand.quantity;
        self.demand_queue.pop_front();

        for (i, &cost) in reaction.cost.iter().enumerate() {
            if cost > 0 {
                self.total_cost[i] += cost * multiple;
                match i {
                    ORE_IDX => (),
                    FUEL_IDX => panic!("This looks recursive..."),
                    _ => self.add_demand(i, cost * multiple),
                }
            }
        }

        if self.demand_queue.is_empty() {
            ApplyResult::AllDemandsMet
        } else {
            ApplyResult::ContinueSearching
        }
    }
}

fn acquire_fuel(input: &InputInterpretation, initial: SearchState) -> SearchState {
    let mut state = initial;

    loop {
        state.consume_overflow();

        let demand = match state.get_next_demand() {
            DemandType::NeedsOutput(d) => d,
            DemandType::NoFurtherDemands => return state,
        };

        let reaction = &input.reactions[demand.compound_idx];

        match state.apply_reaction(reaction) {
            ApplyResult::AllDemandsMet => {
                return state;
            }
            ApplyResult::ContinueSearching => {
                continue;
            }
        }
    }
}

fn calculate_part_1(input: &InputInterpretation) -> usize {
    let initial = SearchState::initial(input);
    let result = acquire_fuel(input, initial);
    result.ore_cost()
}

// This algorithm might not be correct for all possible inputs.
//
// It works by calculating the "amortised cost" of a single unit of fuel, as a
// fractional amount of ore. It's amortised because we assume that all
// over-production of a compound is re-distributed in to future units of fuel
// with perfect efficiency.
//
// With the amortised cost calculated, we can divide the trillion ore through
// it, and then round down to the nearest integer.
fn calculate_part_2(input: &InputInterpretation) -> usize {
    fn get_amortised_cost_recursive(
        input: &InputInterpretation,
        memo: &mut HashMap<usize, f64>,
        compound: usize,
    ) -> f64 {
        if compound == ORE_IDX {
            return 1f64;
        }

        if memo.contains_key(&compound) {
            return memo[&compound];
        }

        let recipe = &input.reactions[compound];
        let cost: f64 = recipe
            .cost
            .iter()
            .enumerate()
            .map(|(idx, &cost)| {
                if cost > 0 {
                    get_amortised_cost_recursive(input, memo, idx) * (cost as f64)
                } else {
                    0f64
                }
            })
            .sum();

        // Amortise this cost by pretending we can consume fractional amounts
        // of ore, and that all recipies produce exactly one unit of compound.
        let cost = cost / (recipe.output.quantity as f64);

        memo.insert(compound, cost);
        cost
    }

    let mut memo: HashMap<usize, f64> = HashMap::new();

    let per_fuel = get_amortised_cost_recursive(input, &mut memo, FUEL_IDX);
    let fuels = (PART_TWO_ORE as f64) / per_fuel;

    fuels as usize
}

fn inputs() -> InputInterpretation {
    let text = crate::util::read_file("inputs/day14.txt");
    InputInterpretation::parse_input(&text).unwrap()
}

pub fn run_day_fourteen() {
    let input = inputs();
    println!("Day 14, Part 1: {}", calculate_part_1(&input));
    println!("Day 14, Part 2: {}", calculate_part_2(&input));
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
    assert_eq!(31, calculate_part_1(&input));
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
    assert_eq!(165, calculate_part_1(&input));
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
    assert_eq!(13312, calculate_part_1(&input));
    assert_eq!(82892753, calculate_part_2(&input));
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
    assert_eq!(180697, calculate_part_1(&input));
    assert_eq!(5586022, calculate_part_2(&input));
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
    assert_eq!(2210736, calculate_part_1(&input));
    assert_eq!(460664, calculate_part_2(&input));
}

#[test]
fn actual_part_1() {
    let input = inputs();
    assert_eq!(431448, calculate_part_1(&input));
}

#[test]
fn actual_part_2() {
    let input = inputs();
    assert_eq!(3279311, calculate_part_2(&input));
}
