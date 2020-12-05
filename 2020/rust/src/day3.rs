use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: HashSet<(usize, usize)>,
}

fn parse_map(text: &str) -> Option<Map> {
    fn add_trees(line: &str, trees: &mut HashSet<(usize, usize)>, y: usize) {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                trees.insert((x, y));
            }
        }
    }

    text.lines().map(|l| l.trim()).filter(|l| l.len() > 0).fold(
        None,
        |map: Option<Map>, line: &str| match map {
            None => {
                let mut map = Map {
                    width: line.len(),
                    height: 1,
                    trees: HashSet::new(),
                };
                add_trees(line, &mut map.trees, 0);
                Some(map)
            }
            Some(mut map) => {
                if line.len() != map.width {
                    panic!("Unexpected line width");
                }
                add_trees(line, &mut map.trees, map.height);
                map.height += 1;
                Some(map)
            }
        },
    )
}

const PART_ONE_STEP: (usize, usize) = (3, 1);

fn calculate_path(map: &Map, step: &(usize, usize)) -> usize {
    let mut position = (0, 0);
    let mut trees = 0;

    while position.1 < map.height {
        let wrapped_position = (position.0 % map.width, position.1 % map.height);

        if map.trees.contains(&wrapped_position) {
            trees += 1;
        }

        position = (position.0 + step.0, position.1 + step.1);
    }

    trees
}

fn calculate_all_paths(map: &Map) -> usize {
    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    steps.iter().map(|step| calculate_path(map, step)).product()
}

pub fn run_day_three() {
    let inputs = inputs();
    let map = parse_map(&inputs).unwrap();
    println!("Day 3, Part 1: {}", calculate_path(&map, &PART_ONE_STEP));
    println!("Day 3, Part 2: {}", calculate_all_paths(&map));
}

fn inputs() -> String {
    crate::util::read_file("inputs/day3.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#";

        let map = parse_map(text).unwrap();
        assert_eq!(7, calculate_path(&map, &PART_ONE_STEP));
        assert_eq!(336, calculate_all_paths(&map));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let map = parse_map(&inputs).unwrap();
        assert_eq!(237, calculate_path(&map, &PART_ONE_STEP));
        assert_eq!(2106818610, calculate_all_paths(&map));
    }
}
