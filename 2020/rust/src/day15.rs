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

fn calculate_with_end_index(inputs: &[usize], end_at: usize) -> usize {
    let mut last: HashMap<usize, usize> = HashMap::new();
    let mut unpeek: Option<usize> = None;

    for (i, &n) in inputs.iter().enumerate() {
        unpeek = last.insert(n, i + 1);
    }

    let mut i = inputs.len() + 1;
    let mut prev = inputs.last().copied().unwrap();

    while i <= end_at {
        let next = match unpeek {
            None => 0,
            Some(j) => (i - 1) - j,
        };

        unpeek = last.insert(next, i);

        prev = next;
        i += 1;
    }

    prev
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
