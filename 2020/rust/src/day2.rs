use regex::Regex;

lazy_static! {
    static ref RULE_PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

#[derive(Debug, PartialEq)]
struct RuleLine {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

fn parse_rule_line(line: &str) -> Option<RuleLine> {
    let found = RULE_PATTERN.captures(line.trim())?;
    let min = found.get(1)?.as_str().parse::<usize>().ok()?;
    let max = found.get(2)?.as_str().parse::<usize>().ok()?;
    let char = found.get(3)?.as_str().chars().next()?;
    let password = found.get(4)?.as_str().to_owned();

    Some(RuleLine {
        min,
        max,
        char,
        password,
    })
}

fn validation_rule_one(rule: &RuleLine) -> bool {
    let actual = rule.password.chars().filter(|c| c == &rule.char).count();
    return actual >= rule.min && actual <= rule.max;
}

fn validation_rule_two(rule: &RuleLine) -> bool {
    rule.password
        .chars()
        .enumerate()
        .filter(|(i, c)| {
            let idx = i + 1;
            (idx == rule.min || idx == rule.max) && c == &rule.char
        })
        .count()
        == 1
}

fn count_valid_lines<F: FnMut(&RuleLine) -> bool>(text: &str, validator: F) -> i32 {
    text.lines()
        .filter_map(parse_rule_line)
        .filter(validator)
        .count() as i32
}

fn inputs() -> String {
    crate::util::read_file("inputs/day2.txt")
}

pub fn run_day_two() {
    let inputs = inputs();
    println!(
        "Day 2, Part 1: {}",
        count_valid_lines(&inputs, validation_rule_one)
    );
    println!(
        "Day 2, Part 2: {}",
        count_valid_lines(&inputs, validation_rule_two)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";

        assert_eq!(2, count_valid_lines(text, validation_rule_one));
        assert_eq!(1, count_valid_lines(text, validation_rule_two));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(614, count_valid_lines(&inputs, validation_rule_one));
        assert_eq!(354, count_valid_lines(&inputs, validation_rule_two));
    }
}
