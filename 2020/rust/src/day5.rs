use regex::Regex;

pub fn run_day_five() {
    let inputs = inputs();
    println!("Day 5, Part 1: {}", get_largest_id(&inputs));
    println!("Day 5, Part 2: {}", find_gap_id(&inputs));
}

lazy_static! {
    static ref SEAT_PATTERN: Regex = Regex::new(r"^([FB]+)([LR]+)$").unwrap();
}

fn parse_seat_number(text: &str) -> Option<(u8, u8, u16)> {
    let captures = SEAT_PATTERN.captures(text)?;
    let row: u8 = captures
        .get(1)?
        .as_str()
        .chars()
        .fold(0u8, |num, c| match c {
            'F' => num << 1,
            'B' => (num << 1) + 1,
            _ => panic!("Invalid binary digit in row"),
        });

    let col: u8 = captures
        .get(2)?
        .as_str()
        .chars()
        .fold(0u8, |num, c| match c {
            'L' => num << 1,
            'R' => (num << 1) + 1,
            _ => panic!("Invalid binary digit in col"),
        });

    let id: u16 = (row as u16) * 8 + (col as u16);

    Some((row, col, id))
}

fn get_largest_id(text: &str) -> u16 {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| parse_seat_number(l).expect("Expected valid seat").2)
        .max()
        .expect("No seat numbers found")
}

fn find_gap_id(text: &str) -> u16 {
    let mut seats: Vec<u16> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| parse_seat_number(l).expect("Expected valid seat").2)
        .collect();

    seats.sort();
    let mut prev = seats[0];
    for id in seats.iter().skip(1) {
        if *id > prev + 1 {
            return prev + 1;
        }

        prev = *id;
    }

    panic!("Didn't find our seat");
}

fn inputs() -> String {
    crate::util::read_file("inputs/day5.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!((44, 5, 357), parse_seat_number("FBFBBFFRLR").unwrap());
        assert_eq!((70, 7, 567), parse_seat_number("BFFFBBFRRR").unwrap());
        assert_eq!((102, 4, 820), parse_seat_number("BBFFBBFRLL").unwrap());
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(908, get_largest_id(&inputs));
        assert_eq!(619, find_gap_id(&inputs));
    }
}
