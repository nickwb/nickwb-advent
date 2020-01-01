use crate::util;
use std::collections::{HashMap, VecDeque};

struct Body {
    label: String,
    parent: String,
}

type Bodies = Vec<Body>;

fn parse_bodies<'a, I: Iterator<Item = &'a str>>(lines: I) -> Bodies {
    let mut result: Bodies = Vec::new();

    for s in lines {
        let mut parts = s.split(')');
        let parent = parts.next().unwrap();
        let label = parts.next().unwrap();
        assert_eq!(None, parts.next());

        result.push(Body {
            label: label.trim().into(),
            parent: parent.trim().into(),
        });
    }

    result
}

struct OrbitState<'a> {
    bodies: &'a Bodies,
    lookup: HashMap<&'a str, &'a Body>,
    children: HashMap<&'a str, Vec<&'a Body>>,
    total: usize,
    ancestor_count: HashMap<&'a str, usize>,
    deferred: HashMap<&'a str, Vec<&'a Body>>,
}

impl<'a> OrbitState<'a> {
    fn new(bodies: &'a Bodies) -> OrbitState<'a> {
        OrbitState {
            bodies: bodies,
            lookup: HashMap::new(),
            children: HashMap::new(),
            total: 0,
            ancestor_count: HashMap::new(),
            deferred: HashMap::new(),
        }
    }

    fn resolve(&mut self, body: &'a Body, container: Option<&mut Vec<&'a Body>>) {
        let parent_key: &str = &body.parent;
        let my_key: &str = &body.label;
        let parent_count = self.ancestor_count[parent_key];
        self.ancestor_count.insert(my_key, parent_count + 1);
        self.total += parent_count;

        self.children.entry(parent_key).or_default().push(body);

        // Anyone waiting for this result?
        if let Some((_, mut to_resolve)) = self.deferred.remove_entry(my_key) {
            let next = match container {
                Some(c) => {
                    c.append(&mut to_resolve);
                    c
                }
                None => &mut to_resolve,
            };
            while let Some(dependent) = next.pop() {
                self.resolve(dependent, Some(next));
            }
        }
    }

    fn map(&mut self) {
        self.ancestor_count.insert("COM", 1usize);
        for b in self.bodies.iter() {
            let parent_key: &str = &b.parent;
            let my_key: &str = &b.label;
            self.lookup.insert(my_key, b);

            if !self.ancestor_count.contains_key(&parent_key) {
                self.deferred.entry(parent_key).or_default().push(b);
            } else {
                self.resolve(b, None);
            }
        }
    }

    fn get_parent(&self, body: &'a Body) -> Option<&'a Body> {
        let key: &'a str = &body.parent;
        Some(*(self.lookup.get(key)?))
    }

    fn find_path(&self, from: &Body, to: &Body) -> usize {
        let mut to_check: VecDeque<(&Body, usize)> = VecDeque::new();
        to_check.push_back((from, 0usize));
        to_check.push_back((to, 0usize));
        let mut seen: HashMap<&str, usize> = HashMap::new();

        while let Some((body, cost)) = to_check.pop_front() {
            let my_key: &str = &body.label;
            if let Some(join_cost) = seen.get(my_key) {
                return join_cost + cost;
            }

            seen.insert(my_key, cost);
            if let Some(next_body) = self.get_parent(body) {
                let next = (next_body, cost + 1);
                to_check.push_back(next);
            }
        }

        panic!("Got to the end, and no result")
    }
}

fn input() -> Bodies {
    parse_bodies(util::read_file("inputs/day6.txt").lines())
}

fn calculate_day_six() -> (usize, usize) {
    let bodies = input();
    let mut state = OrbitState::new(&bodies);
    state.map();

    let you = &state.lookup["YOU"];
    let san = &state.lookup["SAN"];
    let path = state.find_path(*you, *san) - 2;

    (state.total, path)
}

pub fn run_day_six() {
    let (part_one, part_two) = calculate_day_six();
    println!("Day 6, Part 1: {}", part_one);
    println!("Day 6, Part 2: {}", part_two);
}

#[test]
fn example_1() {
    let input = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];
    let bodies = parse_bodies(input.iter().copied());
    let mut state = OrbitState::new(&bodies);
    state.map();

    assert_eq!(11, bodies.len());
    assert_eq!(42, state.total);
}

#[test]
fn actual_day_six() {
    let (part_one, part_two) = calculate_day_six();
    assert_eq!(227612, part_one);
    assert_eq!(454, part_two);
}
