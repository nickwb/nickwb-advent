use crate::intcode::*;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    mem,
    rc::Rc,
};

pub fn run_day_fifteen() {
    let mut droid = Droid::new(inputs());
    droid.explore_whole_map_dfs();
    let (depth, oxygen_location) = droid.oxygen_bfs().unwrap();
    println!("Day 15, Part 1: {}", depth);
    let fill_time = droid.flood_fill(oxygen_location);
    println!("Day 15, Part 2: {}", fill_time);
}

fn inputs() -> Vec<MemoryCell> {
    crate::util::read_int_array("inputs/day15.txt")
}

type DroidComputer = Computer<Vec<MemoryCell>, Rc<RefCell<DroidIo>>, Rc<RefCell<DroidIo>>>;

struct Droid {
    computer: DroidComputer,
    map: HashMap<Point, Observation>,
    location: Point,
}

impl Droid {
    fn new(program: Vec<MemoryCell>) -> Droid {
        let io = Rc::new(RefCell::new(DroidIo {
            next_input: None,
            next_output: None,
        }));

        let mut computer: DroidComputer = Computer::new(program, io.clone(), io);
        computer.enable_extra_memory();

        Self {
            computer,
            map: HashMap::new(),
            location: ZERO_POINT,
        }
    }

    fn explore_whole_map_dfs(&mut self) {
        // Mark 0,0 as explored
        self.map.insert(ZERO_POINT, Observation::Empty);

        let mut candidates: Vec<Point> = Vec::new();

        // Initialise candidates with the neighbours of 0,0
        for (_d, n) in Self::neighbours(&ZERO_POINT) {
            candidates.push(n);
        }

        // Track our path so we can backtrack if required
        let mut path: Vec<Direction> = Vec::new();

        while let Some(dest) = candidates.pop() {
            // Ignore candidates we've already explored
            if self.map.contains_key(&dest) {
                continue;
            }

            // Backtrack if necessary
            let direction = loop {
                match self.is_adjacent(&dest) {
                    None => {
                        let direction = path.pop().unwrap();
                        self.traverse(match direction {
                            Direction::North => Direction::South,
                            Direction::South => Direction::North,
                            Direction::East => Direction::West,
                            Direction::West => Direction::East,
                        });
                    }
                    Some(d) => break d,
                }
            };

            // Visit the new location
            let observation = self.traverse(direction);

            self.map.insert(dest, observation);

            // If we didn't just ram the wall, add this movement to the backtrack
            if observation != Observation::Wall {
                path.push(direction);
            }

            // Add each new neighbour as a candidate
            for (_d, n) in Self::neighbours(&self.location) {
                // Ignore the neigbour if we've already seen it
                if self.map.contains_key(&n) {
                    continue;
                }

                // Mark that cell for exploring
                candidates.push(n);
            }
        }
    }

    fn traverse(&mut self, direction: Direction) -> Observation {
        let input = match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        };

        {
            let mut io = self.computer.input().borrow_mut();
            io.next_input = Some(input);
        }

        let result = self.computer.resume();
        match result {
            Ok(StepResult::WaitingOnInput) => (),
            _ => panic!("Computer didn't traverse as expected"),
        }

        let observation = {
            let mut io = self.computer.output().borrow_mut();
            io.next_output.take()
        };

        let observation = match observation {
            Some(0) => Observation::Wall,
            Some(1) => Observation::Empty,
            Some(2) => Observation::OxygenSystem,
            Some(_) => panic!("Unexpected output from computer"),
            None => panic!("Computer didn't produce output"),
        };

        if observation != Observation::Wall {
            let movement = match direction {
                Direction::North => Point::xy(0, -1),
                Direction::South => Point::xy(0, 1),
                Direction::East => Point::xy(1, 0),
                Direction::West => Point::xy(-1, 0),
            };
            self.location = self.location + movement;
        }

        observation
    }

    fn neighbours<'a>(location: &'a Point) -> impl Iterator<Item = (Direction, Point)> + 'a {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        DIRECTIONS.iter().map(move |&d| match d {
            Direction::North => (d, Point::xy(location.x, location.y - 1)),
            Direction::South => (d, Point::xy(location.x, location.y + 1)),
            Direction::East => (d, Point::xy(location.x + 1, location.y)),
            Direction::West => (d, Point::xy(location.x - 1, location.y)),
        })
    }

    fn is_adjacent(&self, location: &Point) -> Option<Direction> {
        Self::neighbours(&self.location)
            .filter_map(|(d, l)| if &l == location { Some(d) } else { None })
            .next()
    }

    // Find the optimal path to the oxygen from the origin,
    // returning the moves taken and the location of the oxygen
    fn oxygen_bfs(&self) -> Option<(usize, Point)> {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut candidates: VecDeque<(usize, Point)> = VecDeque::new();
        candidates.push_front((0, ZERO_POINT));

        while let Some((length, position)) = candidates.pop_back() {
            if visited.contains(&position) {
                continue;
            }

            visited.insert(position);

            for (_d, n) in Self::neighbours(&position) {
                if visited.contains(&n) {
                    continue;
                }

                let x = (length + 1, n);
                match self.map[&n] {
                    Observation::OxygenSystem => return Some(x),
                    Observation::Empty => candidates.push_front(x),
                    _ => {} // We can't traverse walls, so they not be part of the shortest path
                };
            }
        }
        None
    }

    // Count how many epochs are required to flood fill all the empty space, start from a starting point
    fn flood_fill(&self, from: Point) -> usize {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut this_batch: Vec<Point> = Vec::new();
        let mut next_batch: Vec<Point> = Vec::new();
        let mut epoch = 0;
        this_batch.push(from);

        while !this_batch.is_empty() {
            seen.extend(&this_batch);

            // Yeah, we can end up with some duplicates in this, but removing them is not significantly faster
            next_batch.extend(
                this_batch
                    .iter()
                    .flat_map(|p| Self::neighbours(p))
                    .filter_map(|(_d, n)| {
                        let observation = self.map.get(&n)?;
                        if !seen.contains(&n) && observation == &Observation::Empty {
                            Some(n)
                        } else {
                            None
                        }
                    }),
            );

            this_batch.clear();
            mem::swap(&mut this_batch, &mut next_batch);
            epoch += 1;
        }

        epoch - 1
    }
}

struct DroidIo {
    next_input: Option<MemoryCell>,
    next_output: Option<MemoryCell>,
}

impl InputSource for Rc<RefCell<DroidIo>> {
    fn next(&mut self) -> Option<MemoryCell> {
        let mut io = self.borrow_mut();
        io.next_input.take()
    }
}

impl OutputSink for Rc<RefCell<DroidIo>> {
    fn write(&mut self, value: MemoryCell) {
        let mut io = self.borrow_mut();
        io.next_output = Some(value);
    }
}

type Point = crate::util::Point<MemoryCell>;

const ZERO_POINT: Point = Point::xy(0, 0);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Observation {
    Empty,
    Wall,
    OxygenSystem,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_inputs() {
        let mut droid = Droid::new(inputs());
        droid.explore_whole_map_dfs();
        let (depth, oxygen_location) = droid.oxygen_bfs().unwrap();
        assert_eq!(336, depth);
        let fill_time = droid.flood_fill(oxygen_location);
        assert_eq!(360, fill_time);
    }
}
