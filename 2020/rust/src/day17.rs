use std::{collections::HashMap, hash::Hash};

pub fn run_day_seventeen() {
    let grid_one = inputs();
    let grid_two = grid_one.clone();
    println!("Day 17, Part 1: {}", calculate_part_1(grid_one));
    println!("Day 17, Part 2: {}", calculate_part_2(grid_two));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Debug, Clone)]
struct ActiveGrid {
    cells: Vec<Point>,

    // candidates and swap_cells are just collections that we recycle to avoid their re-allocation
    candidates: HashMap<Point, usize>,
    swap_cells: Option<Vec<Point>>,
}

impl ActiveGrid {
    fn from_inputs(cells: Vec<Point>) -> Self {
        Self {
            cells,
            candidates: HashMap::with_capacity(80),
            swap_cells: None,
        }
    }
}

fn calculate_part_1(input: ActiveGrid) -> usize {
    let mut grid = input;
    for _ in 0..6 {
        grid = run_conway_cycle(grid, true)
    }
    grid.cells.len()
}

fn calculate_part_2(input: ActiveGrid) -> usize {
    let mut grid = input;
    for _ in 0..6 {
        grid = run_conway_cycle(grid, false)
    }
    grid.cells.len()
}

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
            w: 0,
        })
        .collect::<Vec<Point>>();

    ActiveGrid::from_inputs(cells)
}

fn run_conway_cycle(grid: ActiveGrid, suppress_w: bool) -> ActiveGrid {
    let ActiveGrid {
        mut cells,
        mut candidates,
        swap_cells,
    } = grid;

    let mut next = swap_cells.unwrap_or(Vec::new());

    for (i, cell) in cells.iter().enumerate() {
        // Find candidate neighbors that could be promoted to active
        for x in cell.x - 1..=cell.x + 1 {
            for y in cell.y - 1..=cell.y + 1 {
                for z in cell.z - 1..=cell.z + 1 {
                    let w_range = if suppress_w {
                        (0, 0)
                    } else {
                        (cell.w - 1, cell.w + 1)
                    };
                    for w in w_range.0..=w_range.1 {
                        // This check is not technically necessary, given that we cull the
                        // candidates list later. But a quick benchmark suggests that including
                        // this check is marginally more efficient.
                        if x == cell.x && y == cell.y && z == cell.z && w == cell.w {
                            continue;
                        }

                        candidates
                            .entry(Point { x, y, z, w })
                            .and_modify(|n| *n += 1)
                            .or_insert(1);
                    }
                }
            }
        }

        // Check if this cell stays active by checking whether it neighbors other active cells
        let mut n = 0;
        for (j, other) in cells.iter().enumerate() {
            if i == j {
                continue;
            }

            if other.x >= cell.x - 1
                && other.x <= cell.x + 1
                && other.y >= cell.y - 1
                && other.y <= cell.y + 1
                && other.z >= cell.z - 1
                && other.z <= cell.z + 1
                && other.w >= cell.w - 1
                && other.w <= cell.w + 1
            {
                n += 1;
            }
        }

        // Does this cell remain active?
        if n == 2 || n == 3 {
            next.push(*cell);
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

        let grid_one = parse_initial_grid(text);
        let grid_two = grid_one.clone();
        assert_eq!(112, calculate_part_1(grid_one));
        assert_eq!(848, calculate_part_2(grid_two));
    }

    #[test]
    fn actual_inputs() {
        let grid_one = inputs();
        let grid_two = grid_one.clone();
        assert_eq!(291, calculate_part_1(grid_one));
        assert_eq!(1524, calculate_part_2(grid_two));
    }
}
