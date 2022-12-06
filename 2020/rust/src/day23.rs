use std::collections::HashMap;

pub fn run_day_twenty_three() {
    let part_1 = calculate_part_1("853192647");
    println!("Day 22, Part 1: {}", part_1);
}

fn calculate_part_1(input: &str) -> String {
    let mut cups = Cups::from_input(input);
    cups.play_all_rounds(9, 100);
    cups.part_one()
}

#[derive(Debug)]
struct Cups {
    next: HashMap<u32, u32>,
    current: u32,
}

impl Cups {
    fn from_input(digits: &str) -> Self {
        let digits: Vec<u32> = digits
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u32))
            .collect();

        let last = digits.len() - 1;
        let mut next = HashMap::new();

        for i in 0..=last {
            let d = digits[i];
            next.insert(d, if i == last { digits[0] } else { digits[i + 1] });
        }

        Self {
            next,
            current: digits[0],
        }
    }

    fn play_all_rounds(&mut self, cup_count: u32, round_count: u32) {
        for _ in 0..round_count {
            self.play_one_round(cup_count);
        }
    }

    fn play_one_round(&mut self, cup_count: u32) {
        debug_assert_eq!(cup_count, self.next.len() as u32);

        // Find the cups to be removed
        let current = self.current;
        let a = self.next[&self.current];
        let b = self.next[&a];
        let c = self.next[&b];
        let rejoin = self.next[&c];

        // Remove the cups
        self.next.insert(current, rejoin);
        self.next.remove(&c);

        // Find the destination
        let mut dest = current - 1;
        loop {
            if dest == 0 {
                dest = cup_count;
                continue;
            }

            if dest == a || dest == b || dest == c {
                dest -= 1;
                continue;
            }

            break;
        }

        // Insert the cups back in
        let rejoin = self.next[&dest];
        self.next.insert(dest, a);
        self.next.insert(c, rejoin);

        // New current cup
        self.current = self.next[&current];

        debug_assert_eq!(cup_count, self.next.len() as u32);
    }

    fn part_one(&self) -> String {
        let mut buffer = String::with_capacity(8);

        fn to_digit(n: u32) -> char {
            let n = n as u8;
            (n + 48) as char
        }

        let mut next = self.next[&1];
        while next != 1 {
            buffer.push(to_digit(next));
            next = self.next[&next]
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let part_1 = calculate_part_1("389125467");
        assert_eq!("67384529", part_1);
    }

    #[test]
    fn actual_inputs() {
        let part_1 = calculate_part_1("853192647");
        assert_eq!("97624853", part_1);
    }
}
