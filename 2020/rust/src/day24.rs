use std::collections::HashMap;

pub fn run_day_twenty_four() {
    let inputs = inputs();
    let (part_1, part_2) = calculate_both_parts(&inputs);
    println!("Day 24, Part 1: {}", part_1);
    println!("Day 24, Part 2: {}", part_2);
}

fn calculate_both_parts(inputs: &Inputs) -> (usize, usize) {
    let mut floor = TiledFloor::from_inputs(inputs);
    let part_1 = floor.count_black_tiles();
    for _i in 0..100 {
        floor.single_day_transform();
    }
    let part_2 = floor.count_black_tiles();
    (part_1, part_2)
}

struct TiledFloor {
    tiles: HashMap<Point, TileColor>,
}

impl TiledFloor {
    fn from_inputs(inputs: &Inputs) -> Self {
        let mut tiles: HashMap<Point, TileColor> = HashMap::new();
        for p in &inputs.flip_paths {
            let mut point = Point::from_qr(0, 0);
            for &d in &p.steps {
                point = point.step(d, 1);
            }

            let tile = tiles.entry(point).or_insert(TileColor::White);
            *tile = match tile {
                TileColor::Black => TileColor::White,
                TileColor::White => TileColor::Black,
            };
        }
        Self { tiles }
    }

    fn single_day_transform(&mut self) {
        let black_tiles: Vec<Point> = self
            .tiles
            .iter()
            .filter_map(
                |(&p, &t)| {
                    if t == TileColor::Black {
                        Some(p)
                    } else {
                        None
                    }
                },
            )
            .collect();

        let mut updates: Vec<(Point, TileColor)> = Vec::new();
        for p in &black_tiles {
            let mut black_neighbour_count = 0;
            for n in p.neighbours() {
                let tile = self.tiles.entry(n).or_insert(TileColor::White);
                if tile == &TileColor::Black {
                    black_neighbour_count += 1;
                }
            }
            if black_neighbour_count == 0 || black_neighbour_count > 2 {
                updates.push((*p, TileColor::White))
            }
        }

        let white_tiles =
            self.tiles.iter().filter_map(
                |(p, &t)| {
                    if t == TileColor::White {
                        Some(p)
                    } else {
                        None
                    }
                },
            );

        for p in white_tiles {
            let mut black_neighbour_count = 0;
            for n in p.neighbours() {
                let tile = self.tiles.get(&n).unwrap_or(&TileColor::White);
                if tile == &TileColor::Black {
                    black_neighbour_count += 1;
                }
            }
            if black_neighbour_count == 2 {
                updates.push((*p, TileColor::Black));
            }
        }

        for (p, t) in updates {
            self.tiles.insert(p, t);
        }
    }

    fn count_black_tiles(&self) -> usize {
        self.tiles
            .values()
            .filter(|&&v| v == TileColor::Black)
            .count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileColor {
    Black,
    White,
}

#[derive(Debug)]
struct Inputs {
    flip_paths: Vec<Path>,
}

impl Inputs {
    fn parse(text: &str) -> Self {
        let flip_paths: Vec<Path> = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| Path::parse(l).expect("Line must be a valid path"))
            .collect();

        Self { flip_paths }
    }
}

#[derive(Debug)]
struct Path {
    steps: Vec<Direction>,
}

impl Path {
    fn parse(text: &str) -> Option<Self> {
        let mut steps: Vec<Direction> = Vec::new();
        let mut remaining = text;
        while remaining.len() > 0 {
            let (r, d) = Direction::parse(remaining)?;
            steps.push(d);
            remaining = r;
        }

        Some(Self { steps })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn parse<'a>(text: &'a str) -> Option<(&'a str, Self)> {
        let mut chars = text.chars();
        let first = chars.next()?;
        match first {
            'e' => Some((&text[1..], Self::East)),
            'w' => Some((&text[1..], Self::West)),
            'n' => {
                let second = chars.next()?;
                match second {
                    'w' => Some((&text[2..], Self::NorthWest)),
                    'e' => Some((&text[2..], Self::NorthEast)),
                    _ => None,
                }
            }
            's' => {
                let second = chars.next()?;
                match second {
                    'w' => Some((&text[2..], Self::SouthWest)),
                    'e' => Some((&text[2..], Self::SouthEast)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    q: i64,
    r: i64,
}

impl Point {
    fn from_qr(q: i64, r: i64) -> Self {
        Self { q, r }
    }

    fn qr(&self) -> (i64, i64) {
        (self.q, self.r)
    }

    fn step(&self, direction: Direction, units: i64) -> Point {
        let (q, r) = self.qr();
        match direction {
            Direction::East => Self::from_qr(q + units, r),
            Direction::SouthEast => Self::from_qr(q, r + units),
            Direction::SouthWest => Self::from_qr(q - units, r + units),
            Direction::West => Self::from_qr(q - units, r),
            Direction::NorthWest => Self::from_qr(q, r - units),
            Direction::NorthEast => Self::from_qr(q + units, r - units),
        }
    }

    fn neighbours<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        const DIRECTIONS: [Direction; 6] = [
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ];

        DIRECTIONS.iter().map(move |&d| self.step(d, 1))
    }
}

fn inputs() -> Inputs {
    let text = crate::util::read_file("inputs/day24.txt");
    Inputs::parse(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew
        ";

        let inputs = Inputs::parse(text);
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(10, part_1);
        assert_eq!(2208, part_2);
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        let (part_1, part_2) = calculate_both_parts(&inputs);
        assert_eq!(500, part_1);
        assert_eq!(4280, part_2);
    }
}
