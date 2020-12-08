
pub fn run_day_five() {
    let inputs = inputs();
    println!("Day 5, Part 1: {}", get_largest_id(&inputs));
    println!("Day 5, Part 2: {}", find_gap_id(&inputs));
}

fn parse_seat_number(text: &str) -> u16 {
    text.chars().fold(0u16, |num, c| match c {
        'F' | 'L' => num << 1,
        'B' | 'R' => (num << 1) + 1,
        _ => panic!("Invalid seat"),
    })
}

fn get_largest_id(text: &str) -> u16 {
    text.lines()
        .filter_map(crate::util::not_blank)
        .map(|l| parse_seat_number(l))
        .max()
        .expect("No seat numbers found")
}

fn find_gap_id(text: &str) -> u16 {
    let mut seats: Vec<u16> = text
        .lines()
        .filter_map(crate::util::not_blank)
        .map(|l| parse_seat_number(l))
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
        assert_eq!(357, parse_seat_number("FBFBBFFRLR"));
        assert_eq!(567, parse_seat_number("BFFFBBFRRR"));
        assert_eq!(820, parse_seat_number("BBFFBBFRLL"));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(908, get_largest_id(&inputs));
        assert_eq!(619, find_gap_id(&inputs));
    }
}
