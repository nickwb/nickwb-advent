use rayon::prelude::*;

pub fn run_day_eleven() {
    let inputs = inputs();
    println!("Day 11, Part 1: {}", calculate_part_1(inputs.clone()));
    println!("Day 11, Part 2: {}", calculate_part_2(inputs));
}

fn calculate_part_1(map: Map) -> usize {
    run_until_stable_returning_occupied(map, 4, &WalkMode::SingleStep)
}

fn calculate_part_2(map: Map) -> usize {
    run_until_stable_returning_occupied(map, 5, &WalkMode::ThroughFloor)
}

fn inputs() -> Map {
    let text = crate::util::read_file("inputs/day11.txt");
    parse(&text)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridCell {
    Floor,
    Unoccupied,
    Occupied,
}

#[derive(Debug, Clone)]
struct Map {
    width: isize,
    height: isize,
    seats: Vec<Vec<GridCell>>,
}

type Position = (isize, isize);

impl Map {
    fn seat(&self, position: Position) -> GridCell {
        self.seats[position.1 as usize][position.0 as usize]
    }

    fn set_seat(&mut self, position: Position, cell: GridCell) {
        self.seats[position.1 as usize][position.0 as usize] = cell;
    }
}

fn parse(text: &str) -> Map {
    let initial = Map {
        width: 0,
        height: 0,
        seats: Vec::new(),
    };

    text.lines()
        .filter_map(crate::util::not_blank)
        .fold(initial, |mut map: Map, line| {
            let row: Vec<GridCell> = line
                .chars()
                .map(|c| match c {
                    'L' => GridCell::Unoccupied,
                    '.' => GridCell::Floor,
                    _ => panic!("Not expected"),
                })
                .collect();

            if map.width == 0 {
                map.width = row.len() as isize;
            } else if line.len() as isize != map.width {
                panic!("Inconsistent line length");
            }

            map.height += 1;
            map.seats.push(row);
            map
        })
}

fn run_until_stable_returning_occupied(
    mut map: Map,
    exit_tolerance: usize,
    walk_mode: &WalkMode,
) -> usize {
    let mut changes = usize::MAX;
    while changes != 0 {
        changes = musical_chairs_step(&mut map, exit_tolerance, walk_mode);
    }

    map.seats
        .iter()
        .flat_map(|row| row.iter().copied())
        .filter(|&c| c == GridCell::Occupied)
        .count()
}

fn musical_chairs_step(map: &mut Map, exit_tolerance: usize, walk_mode: &WalkMode) -> usize {
    let changes = filter_map_grid_cells_parallel(map, |map, pos| {
        let occupied = fold_adjacent_cells(map, pos, walk_mode, 0usize, |sum, pos| {
            match map.seat(pos) {
                GridCell::Occupied => sum + 1,
                _ => sum,
            }
        });

        match map.seat(pos) {
            GridCell::Unoccupied if occupied == 0 => Some((pos, GridCell::Occupied)),
            GridCell::Occupied if occupied >= exit_tolerance => Some((pos, GridCell::Unoccupied)),
            _ => None,
        }
    });

    for change in &changes {
        map.set_seat(change.0, change.1);
    }

    changes.len()
}

fn filter_map_grid_cells_parallel<U: Send, F: Sync + Fn(&Map, Position) -> Option<U>>(
    map: &Map,
    f: F,
) -> Vec<U> {
    (0..map.width)
        .into_par_iter()
        .flat_map(|x| (0..map.height).into_par_iter().map(move |y| (x, y)))
        .filter_map(|pos| f(map, pos))
        .collect()
}

#[derive(Debug, PartialEq)]
enum WalkMode {
    SingleStep,
    ThroughFloor,
}

fn fold_adjacent_cells<U, F: Fn(U, Position) -> U>(
    map: &Map,
    position: Position,
    walk_mode: &WalkMode,
    mut state: U,
    f: F,
) -> U {
    const DIRECTIONS: [Position; 8] = [
        (0, -1),  // Up
        (0, 1),   // Down
        (-1, 0),  // Left
        (1, 0),   // Right
        (-1, -1), // Up Left
        (1, -1),  // Up Right
        (-1, 1),  // Down Left
        (1, 1),   // Down Right
    ];

    for d in DIRECTIONS.iter() {
        let mut adjacent = position;
        let mut walk_failed = true;

        loop {
            let candidate = (adjacent.0 + d.0, adjacent.1 + d.1);

            if candidate.0 < 0
                || candidate.0 >= map.width
                || candidate.1 < 0
                || candidate.1 >= map.height
            {
                break;
            }

            walk_failed = false;
            adjacent = candidate;

            if &WalkMode::SingleStep == walk_mode {
                break;
            }

            if GridCell::Floor != map.seat(adjacent) {
                break;
            }
        }

        if !walk_failed {
            state = f(state, adjacent);
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        ";

        let map = parse(text);
        assert_eq!(37, calculate_part_1(map.clone()));
        assert_eq!(26, calculate_part_2(map));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(2273, calculate_part_1(inputs.clone()));
        assert_eq!(2064, calculate_part_2(inputs));
    }
}
