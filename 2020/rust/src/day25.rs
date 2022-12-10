pub fn run_day_twenty_five() {
    let inputs = Inputs::new(18356117, 5909654);
    let part_1 = calculate_part_1(&inputs);
    println!("Day 25, Part 1: {}", part_1);
}

fn calculate_part_1(inputs: &Inputs) -> usize {
    let card_loop = solve_for_loop(7, inputs.card_pub);
    let key = evaluate(inputs.door_pub, card_loop);
    key
}

struct Inputs {
    card_pub: usize,
    door_pub: usize,
}

fn solve_for_loop(subject: usize, result: usize) -> usize {
    let mut value = 1;
    for i in 1.. {
        value *= subject;
        value %= 20201227;

        if value == result {
            return i;
        }
    }
    unreachable!()
}

fn evaluate(subject: usize, loops: usize) -> usize {
    let mut value = 1;
    for _ in 0..loops {
        value *= subject;
        value %= 20201227;
    }
    value
}

impl Inputs {
    fn new(card_pub: usize, door_pub: usize) -> Self {
        Self { card_pub, door_pub }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let inputs = Inputs::new(5764801, 17807724);
        let part_1 = calculate_part_1(&inputs);
        assert_eq!(14897079, part_1);
    }

    #[test]
    fn actual_inputs() {
        let inputs = Inputs::new(18356117, 5909654);
        let part_1 = calculate_part_1(&inputs);
        assert_eq!(16902792, part_1);
    }
}
