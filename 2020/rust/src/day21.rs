use itertools::Itertools;
use slotmap::{new_key_type, Key, SecondaryMap, SlotMap};
use std::collections::HashSet;

pub fn run_day_twenty_one() {
    let inputs = inputs();
    let (part_1, part_2) = calculate_both_parts(&inputs);
    println!("Day 21, Part 1: {}", part_1);
    println!("Day 21, Part 2: {}", part_2);
}

fn calculate_both_parts(inputs: &Inputs) -> (usize, String) {
    let mut search = SearchSpace::new(inputs);
    search.reduce();

    debug_assert!(search.is_solved());

    let part_1 = search
        .get_safe_ingredients()
        .map(|i| {
            inputs
                .foods
                .iter()
                .filter(|f| f.ingredients.contains(&i))
                .count()
        })
        .sum();

    let part_2 = search
        .resolved
        .iter()
        .map(|(a, &i)| (&inputs.ingredient_map[i], &inputs.allergen_map[a]))
        .sorted_by_key(|&(_, a)| a)
        .map(|(i, _)| i)
        .join(",");

    (part_1, part_2)
}

#[derive(Debug)]
struct SearchSpace<'a> {
    inputs: &'a Inputs,
    possible: SecondaryMap<Allergen, HashSet<Ingredient>>,
    resolved: SecondaryMap<Allergen, Ingredient>,
    allocated: HashSet<Ingredient>,
}

impl<'a> SearchSpace<'a> {
    fn new(inputs: &'a Inputs) -> Self {
        let all_ingredients: HashSet<_> = inputs.ingredient_map.keys().collect();
        let mut possible = SecondaryMap::new();

        for a in inputs.allergen_map.keys() {
            possible.insert(a, all_ingredients.clone());
        }

        let resolved = SecondaryMap::new();
        let allocated = HashSet::new();

        Self {
            inputs,
            possible,
            resolved,
            allocated,
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut did_reduce = false;
            for f in &self.inputs.foods {
                // Consider the ingredients which are not in this food.
                // These ingredients can not possible correspond to any of the listed allergens.
                let other_ingredients: Vec<Ingredient> = self
                    .inputs
                    .ingredient_map
                    .keys()
                    .filter(|i| !f.ingredients.contains(i))
                    .collect();

                // Remove any instance of an other_ingredients which appears in the possible
                // list for each allergen in this food
                'food_allergens: for &a in &f.allergens {
                    if self.resolved.contains_key(a) {
                        continue;
                    }

                    let possible_set = &mut self.possible[a];
                    for o in &other_ingredients {
                        possible_set.remove(o);

                        if possible_set.len() == 1 {
                            let &one = possible_set.iter().next().unwrap();
                            self.resolved.insert(a, one);
                            possible_set.clear();
                            self.allocated.insert(one);
                            did_reduce = true;
                            continue 'food_allergens;
                        }
                    }
                }
            }

            if !did_reduce {
                return;
            }

            did_reduce = false;
            let allocated = &self.allocated;

            // If we allocated an ingredient to an allergen, then that ingredient
            // is no longer a possible candidate for any other allergen.
            for (a, set) in self.possible.iter_mut() {
                if self.resolved.contains_key(a) {
                    continue;
                }

                let starting_len = set.len();
                set.retain(|i| !allocated.contains(i));
                if set.len() != starting_len {
                    did_reduce = true;
                }
            }

            if !did_reduce {
                return;
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.possible.values().all(|s| s.is_empty())
    }

    fn get_safe_ingredients(&'a self) -> impl Iterator<Item = Ingredient> + 'a {
        self.inputs
            .ingredient_map
            .keys()
            .filter(move |i| !self.allocated.contains(i))
    }
}

new_key_type! { struct Ingredient; }
new_key_type! { struct Allergen; }

#[derive(Debug)]
struct Inputs {
    ingredient_map: SlotMap<Ingredient, String>,
    allergen_map: SlotMap<Allergen, String>,
    foods: Vec<Food>,
}

impl Inputs {
    fn parse(text: &str) -> Self {
        let mut ingredient_map: SlotMap<Ingredient, String> = SlotMap::with_key();
        let mut allergen_map: SlotMap<Allergen, String> = SlotMap::with_key();

        fn get_or_insert<T: Key>(map: &mut SlotMap<T, String>, value: &str) -> T {
            for (k, v) in map.iter() {
                if v == value {
                    return k;
                }
            }
            map.insert(value.to_owned())
        }

        let foods: Vec<Food> = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter_map(|l| {
                let split_idx = l.find("(")?;
                let ingredients: Vec<Ingredient> = l[0..split_idx]
                    .split(' ')
                    .filter_map(|i| {
                        if i.is_empty() {
                            None
                        } else {
                            Some(get_or_insert(&mut ingredient_map, i))
                        }
                    })
                    .collect();

                let allergens = l[split_idx..]
                    .strip_prefix("(contains ")?
                    .strip_suffix(")")?;

                let allergens: Vec<Allergen> = allergens
                    .split(", ")
                    .filter_map(|a| {
                        if a.is_empty() {
                            None
                        } else {
                            Some(get_or_insert(&mut allergen_map, a))
                        }
                    })
                    .collect();

                Some(Food {
                    ingredients,
                    allergens,
                })
            })
            .collect();

        Inputs {
            ingredient_map,
            allergen_map,
            foods,
        }
    }
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

fn inputs() -> Inputs {
    let text = crate::util::read_file("inputs/day21.txt");
    Inputs::parse(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        ";

        let inputs = Inputs::parse(text);
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(5, part_1);
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part_2);
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(2798, part_1);
        assert_eq!("gbt,rpj,vdxb,dtb,bqmhk,vqzbq,zqjm,nhjrzzj", part_2);
    }
}
