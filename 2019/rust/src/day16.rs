use rayon::prelude::*;
use std::{convert::TryInto, iter::repeat};

pub fn run_day_sixteen() {
    let text = inputs();
    let part_1 = Digits::parse(&text).calculate_fft(100).to_string(8);
    let part_2 = Digits::parse(&text).solve_part_two();
    println!("Day 16, Part 1: {}", part_1);
    println!("Day 16, Part 2: {}", part_2);
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
            .filter_map(|c| c.to_digit(10).map(|d| d as isize))
            .collect();

        Self {
            spare: Vec::with_capacity(items.len()),
            items,
        }
    }

    fn calculate_fft(mut self, phases: usize) -> Self {
        for _ in 0..phases {
            self.calculate_phase();
        }

        self
    }

    fn calculate_phase(&mut self) {
        let items = &self.items[..];
        let outputs = (0..items.len()).into_par_iter().map(|i| {
            let sum: isize = items
                .iter()
                .zip(multiply_pattern(i + 1))
                .skip(i) // The first i items are definitely zero
                .map(|(&x, y)| x * y)
                .sum();
            last_digit_of(sum)
        });
        self.spare.clear();
        self.spare.par_extend(outputs);

        std::mem::swap(&mut self.items, &mut self.spare);
    }

    fn to_string(&self, n: usize) -> String {
        self.to_string_with_offset(n, 0)
    }

    fn to_string_with_offset(&self, n: usize, offset: usize) -> String {
        let n = if n <= 0 { self.items.len() } else { n };
        let mut buf = String::with_capacity(n);
        buf.extend(
            self.items
                .iter()
                .skip(offset)
                .take(n)
                .filter_map(|&x| char::from_digit(x.try_into().ok()?, 10)),
        );
        buf
    }

    fn solve_part_two(&mut self) -> String {
        let offset = make_number(&self.items[0..7]) as usize;
        let input_len = self.items.len();
        let signal_len = input_len * 10_000;

        // This solution assumes that the offset will have a coefficient of one
        // But this only works if the real signal is shorter than twice the offset
        assert!(offset * 2 > signal_len);
        assert!(offset < signal_len);

        // Pre-allocate our vecs, now that we can predict their required capacities
        let working_items = (signal_len - offset) + 1;
        self.items.reserve_exact(working_items - self.items.len());
        self.spare.reserve_exact(working_items - self.spare.len());

        // Build the real signal, starting from the message offset
        // The formulation of the problem guarantees that all values prior to
        // the offset will have a coefficient of zero, so we can ignore them
        let signal = (offset..signal_len).map(|i| self.items[i % input_len]);
        self.spare.extend(signal);

        std::mem::swap(&mut self.items, &mut self.spare);
        self.spare.clear();

        for _ in 0..100 {
            let mut sum = 0;

            // Due to the pattern offset of 1, the first element still has a coefficient of zero
            self.spare.push(0);
            self.spare.extend(self.items.iter().map(|n| {
                sum += n;
                sum
            }));

            // The extra element due to the pattern offset is not needed
            self.spare.pop();

            // For each digit, the desired new digit will be equal to the sum of all subsequent digits, mod 10.
            // The sum of the current digit, plus all of its preceding digits is currently stored in spare.
            // So, we can calculate the desired digit by subtracting the current value from the total sum.
            self.spare.iter_mut().for_each(|n| {
                let digit = (sum - *n) % 10;
                *n = digit;
            });

            std::mem::swap(&mut self.items, &mut self.spare);
            self.spare.clear();
        }

        self.to_string(8)
    }
}

fn multiply_pattern(seed: usize) -> impl Iterator<Item = isize> {
    const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];
    (0..)
        .flat_map(move |idx| repeat(BASE_PATTERN[idx % 4]).take(seed))
        .skip(1)
}

fn make_number(digits: &[isize]) -> isize {
    digits.iter().fold(0, |num, &x| (num * 10) + x)
}

fn last_digit_of(value: isize) -> isize {
    (value % 10).abs()
}

fn inputs() -> String {
    crate::util::read_file("inputs/day16.txt")
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
    fn example_2() {
        assert_eq!(
            "84462026",
            Digits::parse("03036732577212944063491565474664").solve_part_two()
        );
        assert_eq!(
            "78725270",
            Digits::parse("02935109699940807407585447034323").solve_part_two()
        );
        assert_eq!(
            "53553731",
            Digits::parse("03081770884921959731165446850517").solve_part_two()
        );
    }

    #[test]
    fn actual_inputs() {
        let text = inputs();
        let part_1 = Digits::parse(&text).calculate_fft(100).to_string(8);
        let part_2 = Digits::parse(&text).solve_part_two();
        assert_eq!("74608727", part_1);
        assert_eq!("57920757", part_2);
    }
}
