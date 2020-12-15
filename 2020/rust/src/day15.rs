use std::collections::HashMap;

pub fn run_day_fifteen() {
    let inputs = inputs();
    println!("Day 15, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 15, Part 2: {}", calculate_part_2(&inputs));
}

fn calculate_part_1(inputs: &[usize]) -> usize {
    calculate_with_end_index(inputs, 2020)
}

fn calculate_part_2(inputs: &[usize]) -> usize {
    calculate_with_end_index(inputs, 30000000)
}

const FAST_ALLOCATION: usize = 5000;

fn calculate_with_end_index(inputs: &[usize], end_at: usize) -> usize {
    let mut fast_last: [Option<usize>; FAST_ALLOCATION] = [None; FAST_ALLOCATION];
    let mut extended_last: HashMap<usize, usize> = HashMap::with_capacity(3620000);

    let mut insert_returning_previous = |value, i| {
        if value < FAST_ALLOCATION {
            // Safety: If statement already guarantees we are within bounds
            let result = unsafe { *fast_last.get_unchecked(value) };
            fast_last[value] = Some(i);
            return result;
        }

        extended_last.insert(value, i)
    };

    let mut unpeek: Option<usize> = None;

    for (i, &n) in inputs.iter().enumerate() {
        unpeek = insert_returning_previous(n, i + 1);
    }

    let mut i = inputs.len();

    loop {
        i += 1;

        let next = match unpeek {
            None => 0,
            Some(j) => (i - 1) - j,
        };

        if i == end_at {
            return next;
        }

        unpeek = insert_returning_previous(next, i);
    }
}

fn inputs() -> Vec<usize> {
    let text = crate::util::read_file("inputs/day15.txt");
    parse(&text)
}

fn parse(text: &str) -> Vec<usize> {
    text.trim().split(',').map(|t| t.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = parse("0,3,6");
        assert_eq!(436, calculate_part_1(&input));
        assert_eq!(175594, calculate_part_2(&input));
    }

    #[test]
    #[cfg(slow_problems)]
    fn example_2() {
        assert_eq!(2578, calculate_part_2(&parse("1,3,2")));
        assert_eq!(3544142, calculate_part_2(&parse("2,1,3")));
        assert_eq!(261214, calculate_part_2(&parse("1,2,3")));
        assert_eq!(6895259, calculate_part_2(&parse("2,3,1")));
        assert_eq!(18, calculate_part_2(&parse("3,2,1")));
        assert_eq!(362, calculate_part_2(&parse("3,1,2")));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(536, calculate_part_1(&inputs));
        assert_eq!(24065124, calculate_part_2(&inputs));
    }
}
