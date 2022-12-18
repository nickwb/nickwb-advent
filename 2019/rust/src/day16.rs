use std::{convert::TryInto, iter::repeat};

pub fn run_day_sixteen() {
    let part_1 = inputs().calculate_fft(100).to_string(8);
    println!("Day 16, Part 1: {}", part_1);
    println!("Day 16, Part 2: {}", 0);
}

#[derive(Debug, Clone)]
struct Digits {
    items: Vec<isize>,
    spare: Vec<isize>,
}

impl Digits {
    fn parse(text: &str) -> Self {
        let items: Vec<isize> = text
            .trim()
            .chars()
            .filter_map(|c| {
                if c.is_digit(10) {
                    Some((c as isize) - 48)
                } else {
                    None
                }
            })
            .collect();

        Self {
            spare: items.clone(),
            items,
        }
    }

    fn calculate_fft(&self, phases: usize) -> Digits {
        (0..phases).fold(self.clone(), |mut digits, _| {
            digits.calculate_phase();
            digits
        })
    }

    fn calculate_phase(&mut self) {
        let items = &self.items;
        let outputs = &mut self.spare;
        let len = items.len();
        for i in 0..len {
            outputs[i] = Self::last_digit_of(
                items
                    .iter()
                    .zip(Self::multiply_pattern(i + 1))
                    .map(|(&x, y)| x * y)
                    .sum(),
            );
        }

        std::mem::swap(&mut self.items, &mut self.spare);
    }

    fn multiply_pattern(seed: usize) -> impl Iterator<Item = isize> {
        const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];
        (0..)
            .flat_map(move |idx| repeat(BASE_PATTERN[idx % 4]).take(seed))
            .skip(1)
    }

    fn last_digit_of(value: isize) -> isize {
        (value % 10).abs()
    }

    fn to_string(&self, n: usize) -> String {
        let n = if n <= 0 { self.items.len() } else { n };
        let mut buf = String::with_capacity(n);
        buf.extend(
            self.items
                .iter()
                .take(n)
                .filter_map(|&x| char::from_digit(x.try_into().ok()?, 10)),
        );
        buf
    }
}

fn inputs() -> Digits {
    let text = crate::util::read_file("inputs/day16.txt");
    Digits::parse(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "01029498",
            Digits::parse("12345678").calculate_fft(4).to_string(8)
        );

        assert_eq!(
            "24176176",
            Digits::parse("80871224585914546619083218645595")
                .calculate_fft(100)
                .to_string(8)
        );

        assert_eq!(
            "73745418",
            Digits::parse("19617804207202209144916044189917")
                .calculate_fft(100)
                .to_string(8)
        );

        assert_eq!(
            "52432133",
            Digits::parse("69317163492948606335995924319873")
                .calculate_fft(100)
                .to_string(8)
        );
    }

    #[test]
    fn actual_inputs() {
        let part_1 = inputs().calculate_fft(100).to_string(8);
        assert_eq!("74608727", part_1);
    }
}
