use rayon::prelude::*;
use std::{convert::TryInto, iter::repeat};

pub fn run_day_sixteen() {
    let text = inputs();
    let part_1 = Digits::parse(&text).calculate_fft(100).to_string(8);
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
        Self::parse_and_expand(text, 1)
    }

    fn parse_and_expand(text: &str, repeats: usize) -> Self {
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

        let items = if repeats > 0 {
            let mut repeated = Vec::with_capacity(items.len() * repeats);
            for _ in 0..repeats {
                repeated.extend_from_slice(&items);
            }

            repeated
        } else {
            items
        };

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
        eprintln!("{}", self.to_string(128));
        let items = &self.items[..];
        let outputs = (0..items.len()).into_par_iter().map(|i| {
            let sum = build_single_sum(i + 1, items);
            last_digit_of(sum)

            // let sum: isize = items
            //     .iter()
            //     .zip(multiply_pattern(i + 1))
            //     .map(|(&x, y)| x * y)
            //     .sum();
            // last_digit_of(sum)
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

    fn to_string_from_inline_offset(&self) -> String {
        let offset = self.to_string_with_offset(7, 0);
        let offset: usize = offset.parse().unwrap();
        self.to_string_with_offset(8, offset)
    }
}

fn build_single_sum(repeats: usize, items: &[isize]) -> isize {
    const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

    let mut r = repeats - 1;
    let mut j = 0;
    let mut remaining = items.len();
    let mut sum = 0;
    loop {
        for p in BASE_PATTERN {
            let advance = r.min(remaining);
            let k = j + advance;

            if p != 0 {
                // SAFETY: I pinky promise that it is always in bounds
                let span: isize = unsafe { items.get_unchecked(j..k).iter().sum() };
                //let span: isize = (&items[j..k]).iter().sum();
                sum += p * span;
            }

            // if p == 1 {
            //     sum += (&items[j..k]).iter().sum::<isize>();
            // } else if p == -1 {
            //     sum -= (&items[j..k]).iter().sum::<isize>();
            // }

            j += advance;
            remaining -= advance;

            if remaining == 0 {
                return sum;
            }

            r = repeats;
        }
    }
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
            Digits::parse_and_expand("03036732577212944063491565474664", 10000)
                .calculate_fft(100)
                .to_string_from_inline_offset()
        );
    }

    #[test]
    fn actual_inputs() {
        let text = inputs();
        let part_1 = Digits::parse(&text).calculate_fft(100).to_string(8);
        assert_eq!("74608727", part_1);
    }
}
