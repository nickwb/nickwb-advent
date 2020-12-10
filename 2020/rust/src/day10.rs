use std::collections::HashMap;

pub fn run_day_ten() {
    let inputs = inputs();
    println!("Day 10, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 10, Part 2: {}", calculate_part_2(&inputs));
}

fn inputs() -> Vec<usize> {
    let text = crate::util::read_file("inputs/day10.txt");
    parse(&text)
}

fn calculate_part_1(input: &[usize]) -> usize {
    let mut sorted = Vec::from(input);
    sorted.sort();
    sorted.push(sorted[sorted.len() - 1] + 3);

    let (ones, threes, _) =
        sorted
            .iter()
            .fold((0, 0, None), |(mut ones, mut threes, last), &value| {
                let last = last.unwrap_or(0);
                let diff = value - last;

                match diff {
                    1 => ones += 1,
                    3 => threes += 1,
                    _ => panic!("Not expected"),
                }

                (ones, threes, Some(value))
            });

    ones * threes
}

fn calculate_part_2(input: &[usize]) -> usize {
    let mut sorted: Vec<usize> = Vec::new();
    sorted.push(0);
    sorted.extend(input);
    sorted.sort();
    sorted.push(sorted[sorted.len() - 1] + 3);

    let mut memo = HashMap::new();
    find_permutations_recursive(&sorted, &mut memo)
}

fn find_permutations_recursive(tail: &[usize], memo: &mut HashMap<usize, usize>) -> usize {
    if tail.len() == 1 {
        return 1;
    }

    let value = tail[0];
    let tail = &tail[1..];
    let end_idx = tail.iter().position(|&v| v > value + 3);

    let connected = match end_idx {
        Some(i) => &tail[..i],
        None => tail,
    };

    let result = connected
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let key = tail.len() - i;

            match memo.get(&key) {
                Some(&v) => v,
                None => find_permutations_recursive(&tail[i..], memo),
            }
        })
        .sum();

    memo.insert(tail.len() + 1, result);
    result
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
            16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4
        ";

        let parsed = parse(input);
        assert_eq!(35, calculate_part_1(&parsed));
        assert_eq!(8, calculate_part_2(&parsed));
    }

    #[test]
    fn example_2() {
        let input = r"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        ";

        let parsed = parse(input);
        assert_eq!(220, calculate_part_1(&parsed));
        assert_eq!(19208, calculate_part_2(&parsed));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(2201, calculate_part_1(&inputs));
        assert_eq!(169255295254528, calculate_part_2(&inputs));
    }
}
