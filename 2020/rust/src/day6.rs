use std::collections::{HashMap, HashSet};

pub fn run_day_six() {
    let inputs = inputs();
    let groups = parse_input(&inputs);
    println!("Day 6, Part 1: {}", sum_unique_groups(&groups));
    println!("Day 6, Part 2: {}", sum_matching_groups(&groups));
}

fn inputs() -> String {
    crate::util::read_file("inputs/day6.txt")
}

fn sum_unique_groups(groups: &[TravelGroup]) -> usize {
    groups.iter().map(|g| g.count_unique()).sum()
}

fn sum_matching_groups(groups: &[TravelGroup]) -> usize {
    groups.iter().map(|g| g.count_matching()).sum()
}

struct TravelGroup {
    people: Vec<Person>,
}

impl TravelGroup {
    fn count_unique(&self) -> usize {
        let mut seen: HashSet<char> = HashSet::new();
        for p in self.people.iter() {
            seen.extend(p.questions_true.iter());
        }

        seen.len()
    }

    fn count_matching(&self) -> usize {
        let map = self
            .people
            .iter()
            .flat_map(|p| p.questions_true.iter())
            .fold(HashMap::new(), |mut map, c| {
                if map.contains_key(c) {
                    map.insert(*c, map[c] + 1usize);
                } else {
                    map.insert(*c, 1usize);
                }

                map
            });

        map.iter()
            .filter(|(_, &num)| num == self.people.len())
            .count()
    }
}

struct Person {
    questions_true: HashSet<char>,
}

fn parse_input(text: &str) -> Vec<TravelGroup> {
    let mut result = Vec::new();
    let mut group: Option<TravelGroup> = None;

    for line in text.lines().map(|l| l.trim()) {
        if line.is_empty() {
            if let Some(g) = group.take() {
                result.push(g);
            }
            continue;
        }

        let person = Person {
            questions_true: line.chars().collect(),
        };

        group = match group {
            None => Some(TravelGroup {
                people: vec![person],
            }),
            Some(mut g) => {
                g.people.push(person);
                Some(g)
            }
        }
    }

    if let Some(g) = group.take() {
        result.push(g);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b";

        let groups = parse_input(text);
        assert_eq!(11, sum_unique_groups(&groups));
        assert_eq!(6, sum_matching_groups(&groups));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let groups = parse_input(&inputs);
        assert_eq!(6443, sum_unique_groups(&groups));
        assert_eq!(3232, sum_matching_groups(&groups));
    }
}
