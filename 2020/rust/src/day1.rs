use std::iter::Iterator;

fn calculate_part_1(entries: &[i32]) -> i32 {
    entries
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| {
            entries.iter().enumerate().filter_map(
                move |(j, &y)| {
                    if i == j {
                        None
                    } else {
                        Some((x, y))
                    }
                },
            )
        })
        .filter_map(|(x, y)| if x + y == 2020 { Some(x * y) } else { None })
        .next()
        .unwrap()
}

fn calculate_part_2(entries: &[i32]) -> i32 {
    entries
        .iter()
        .enumerate()
        .flat_map(move |(i, &x)| {
            entries.iter().enumerate().flat_map(move |(j, &y)| {
                entries.iter().enumerate().filter_map(move |(k, &z)| {
                    if i == j || i == k || j == k {
                        None
                    } else {
                        Some((x, y, z))
                    }
                })
            })
        })
        .filter_map(|(x, y, z)| {
            if x + y + z == 2020 {
                Some(x * y * z)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn inputs() -> Vec<i32> {
    crate::util::read_file("inputs/day1.txt")
        .lines()
        .map(|l| l.trim())
        .filter(|s| !s.is_empty())
        .map(|v| match v.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                panic!("Failed to parse a value");
            }
        })
        .collect()
}

pub fn run_day_one() {
    let inputs = inputs();
    println!("Day 1, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 1, Part 2: {}", calculate_part_2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: [i32; 6] = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, calculate_part_1(&input));
        assert_eq!(241861950, calculate_part_2(&input));
    }
}
