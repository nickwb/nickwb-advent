use std::collections::HashMap;

pub fn run_day_twenty_four() {
    let inputs = inputs();
    println!("Day 24, Part 1: {}", calculate_part_1(&inputs));
    // println!("Day 24, Part 1: {}", calculate_part_1(input));
}

fn calculate_part_1(inputs: &Inputs) -> usize {
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

    tiles.values().filter(|&&v| v == TileColor::Black).count()
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
        assert_eq!(10, calculate_part_1(&inputs));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(500, calculate_part_1(&inputs));
    }
}
