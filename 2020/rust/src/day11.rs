use std::collections::HashMap;
use std::iter::FromIterator;

pub fn run_day_eleven() {
    let inputs = inputs();
    println!("Day 10, Part 1: {}", "TODO");
    println!("Day 10, Part 2: {}", "TODO");
}

fn inputs() -> Map {
    let text = crate::util::read_file("inputs/day11.txt");
    parse(&text)
}

type Position = (usize, usize);

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    seats: HashMap<Position, bool>,
}

fn parse(text: &str) -> Map {
    text.lines()
        .filter_map(crate::util::not_blank)
        .fold(None, |map: Option<Map>, line| {
            let y = match map {
                Some(ref m) => m.height,
                None => 0,
            };

            let seats = line.chars().enumerate().filter_map(|(i, c)| match c {
                'L' => Some(((i, y), true)),
                _ => None,
            });

            match map {
                Some(mut map) => {
                    if line.len() != map.width {
                        panic!("Inconsistent line length");
                    }
                    map.height += 1;
                    map.seats.extend(seats);
                    Some(map)
                }
                None => Some(Map {
                    width: line.len(),
                    height: 1,
                    seats: HashMap::from_iter(seats),
                }),
            }
        })
        .unwrap()
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

        //dbg!(parse(text));
    }

    #[test]
    fn actual_inputs() {}
}
