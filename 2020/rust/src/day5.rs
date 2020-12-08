use regex::Regex;

pub fn run_day_five() {
    let inputs = inputs();
    println!("Day 5, Part 1: {}", get_largest_id(&inputs));
    println!("Day 5, Part 2: {}", find_gap_id(&inputs));
}

lazy_static! {
    static ref SEAT_PATTERN: Regex = Regex::new(r"^([FB]+)([LR]+)$").unwrap();
}

fn parse_seat_number(text: &str) -> Option<u16> {
    let captures = SEAT_PATTERN.captures(text)?;
    let row: u16 = captures
        .get(1)?
        .as_str()
        .chars()
        .fold(0u16, |num, c| match c {
            'F' => num << 1,
            'B' => (num << 1) + 1,
            _ => unreachable!("Regex"),
        });

    let col: u16 = captures
        .get(2)?
        .as_str()
        .chars()
        .fold(0u16, |num, c| match c {
            'L' => num << 1,
            'R' => (num << 1) + 1,
            _ => unreachable!("Regex"),
        });

    Some(row * 8 + col)
}

fn get_largest_id(text: &str) -> u16 {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| parse_seat_number(l).expect("Expected valid seat"))
        .max()
        .expect("No seat numbers found")
}

fn find_gap_id(text: &str) -> u16 {
    let mut seats: Vec<u16> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| parse_seat_number(l).expect("Expected valid seat"))
        .collect();

    seats.sort();
    seats.iter().try_fold(None, |last, &seat| { 
        match last {
            None => Ok(Some(seat)),
            Some(x) if seat == x+1 => Ok(Some(seat)),
            Some(x) => Err(x + 1),
        }
     }).expect_err("Seat not found")
}

fn inputs() -> String {
    crate::util::read_file("inputs/day5.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(357, parse_seat_number("FBFBBFFRLR").unwrap());
        assert_eq!(567, parse_seat_number("BFFFBBFRRR").unwrap());
        assert_eq!(820, parse_seat_number("BBFFBBFRLL").unwrap());
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(908, get_largest_id(&inputs));
        assert_eq!(619, find_gap_id(&inputs));
    }
}
