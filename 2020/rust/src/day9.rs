use std::{cmp, collections::VecDeque};

pub fn run_day_nine() {
    let inputs = inputs();
    let part_1 = calculate_part_1(&inputs, 25);
    println!("Day 9, Part 1: {}", part_1);
    println!("Day 9, Part 2: {}", calculate_part_2(&inputs, part_1));
}

fn inputs() -> Vec<usize> {
    let text = crate::util::read_file("inputs/day9.txt");
    parse(&text)
}

fn calculate_part_1(input: &[usize], window_width: usize) -> usize {
    input
        .iter()
        .try_fold(
            VecDeque::with_capacity(window_width),
            |mut window, &value| {
                if window.len() < window_width {
                    window.push_back(value);
                    Ok(window)
                } else if can_be_summed_from_window(value, &window) {
                    window.pop_front();
                    window.push_back(value);
                    Ok(window)
                } else {
                    Err(value)
                }
            },
        )
        .expect_err("Non-summed value not found")
}

fn calculate_part_2(input: &[usize], target_value: usize) -> usize {
    let contiguous = input
        .iter()
        .try_fold(VecDeque::with_capacity(1000), |mut window, &value| {
            let mut sum: usize = window.iter().sum();
            if sum < target_value {
                window.push_back(value);
                Ok(window)
            } else if sum > target_value {
                window.push_back(value);
                sum += value;

                while sum > target_value {
                    sum -= window.pop_front().expect("Not empty");
                }

                Ok(window)
            } else {
                Err(window)
            }
        })
        .expect_err("Contiguous range not found");

    let (min, max) = contiguous
        .iter()
        .fold(None, |min_max, &value| match min_max {
            None => Some((value, value)),
            Some((min, max)) => Some((cmp::min(min, value), cmp::max(max, value))),
        })
        .unwrap();

    min + max
}

fn can_be_summed_from_window(value: usize, window: &VecDeque<usize>) -> bool {
    window
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| {
            window
                .iter()
                .enumerate()
                .filter(move |&(j, _)| i != j)
                .map(move |(_, &b)| a + b)
        })
        .find(|&sum| sum == value)
        .is_some()
}

fn parse(text: &str) -> Vec<usize> {
    text.lines()
        .filter_map(crate::util::not_blank)
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        ";

        let parsed = parse(input);
        assert_eq!(127, calculate_part_1(&parsed, 5));
        assert_eq!(62, calculate_part_2(&parsed, 127));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(1930745883, calculate_part_1(&inputs, 25));
        assert_eq!(268878261, calculate_part_2(&inputs, 1930745883));
    }
}
