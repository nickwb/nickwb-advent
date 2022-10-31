use std::{collections::HashMap, hash::Hash};

pub fn run_day_seventeen() {
    let grid = inputs();
    println!("Day 17, Part 1: {}", calculate_part_1(grid));
    //println!("Day 17, Part 2: {}", calculate_part_2(&mut inputs));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug)]
struct ActiveGrid {
    cells: Vec<Point>,
    candidates: HashMap<Point, usize>,
    swap_cells: Option<Vec<Point>>,
}

impl ActiveGrid {
    fn from_inputs(cells: Vec<Point>) -> Self {
        Self {
            cells,
            candidates: HashMap::with_capacity(26),
            swap_cells: None,
        }
    }
}

fn calculate_part_1(input: ActiveGrid) -> usize {
    let mut grid = input;
    for _ in 0..6 {
        grid = run_single_conway_cycle(grid)
    }
    grid.cells.len()
}

// fn calculate_part_2(input: ActiveGrid) -> usize {
//     0
// }

fn parse_initial_grid(str: &str) -> ActiveGrid {
    let cells = str
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .map(|(x, y)| Point {
            x: x as i32,
            y: y as i32,
            z: 0,
        })
        .collect::<Vec<Point>>();

    ActiveGrid::from_inputs(cells)
}

fn run_single_conway_cycle(grid: ActiveGrid) -> ActiveGrid {
    let ActiveGrid {
        mut cells,
        mut candidates,
        swap_cells,
    } = grid;

    let mut next = swap_cells.unwrap_or(Vec::new());

    for (i, a) in cells.iter().enumerate() {
        // Find candidate neighbors that could be promoted to active
        for x in a.x - 1..=a.x + 1 {
            for y in a.y - 1..=a.y + 1 {
                for z in a.z - 1..=a.z + 1 {
                    if x == a.x && y == a.y && z == a.z {
                        continue;
                    }

                    candidates
                        .entry(Point { x, y, z })
                        .and_modify(|n| *n += 1)
                        .or_insert(1);
                }
            }
        }

        // Check if this cell stays active by checking whether it neighbors other active cells
        let mut n = 0;
        for (j, b) in cells.iter().enumerate() {
            if i == j {
                continue;
            }

            if b.x >= a.x - 1
                && b.x <= a.x + 1
                && b.y >= a.y - 1
                && b.y <= a.y + 1
                && b.z >= a.z - 1
                && b.z <= a.z + 1
            {
                n += 1;
            }
        }

        // Does this cell remain active?
        if n == 2 || n == 3 {
            next.push(*a);
        }
    }

    // Ignore any cell that was previously active
    for a in &cells {
        candidates.remove(a);
    }

    // Promote the inactive candidates to active
    next.extend(
        candidates
            .iter()
            .filter_map(|(p, n)| if *n == 3 { Some(p) } else { None }),
    );

    // Recycle the collections...
    cells.clear();
    candidates.clear();

    ActiveGrid {
        cells: next,
        candidates: candidates,
        swap_cells: Some(cells),
    }
}

fn inputs() -> ActiveGrid {
    let text = crate::util::read_file("inputs/day17.txt");
    parse_initial_grid(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            .#.
            ..#
            ###
        ";

        let grid = parse_initial_grid(text);
        assert_eq!(112, calculate_part_1(grid));
    }

    #[test]
    fn actual_inputs() {
        let grid = inputs();
        assert_eq!(291, calculate_part_1(grid));
    }
}
