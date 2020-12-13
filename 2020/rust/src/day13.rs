pub fn run_day_thirteen() {
    let inputs = inputs();
    println!("Day 13, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 13, Part 2: {}", calculate_part_2(&inputs));
}

fn calculate_part_1(input: &BusInput) -> usize {
    input
        .bus_ids
        .iter()
        .filter_map(|i| match i {
            BusId::Normal(x) => Some(x),
            _ => None,
        })
        .map(|id| {
            let mut min_cycles = input.earliest_timestamp / id;
            let mut arrival = min_cycles * id;

            if arrival < input.earliest_timestamp {
                min_cycles += 1;
                arrival = min_cycles * id;
            }

            let wait = arrival - input.earliest_timestamp;
            (id, wait)
        })
        .min_by_key(|(_, wait)| *wait)
        .map(|(id, wait)| id * wait)
        .expect("Found an answer")
}

struct BusItem {
    id: usize,
    offset: usize,
}

fn calculate_part_2(input: &BusInput) -> usize {
    let sequence: Vec<_> = input
        .bus_ids
        .iter()
        .enumerate()
        .filter_map(|(n, id)| match id {
            BusId::Normal(x) => Some(BusItem { id: *x, offset: n }),
            _ => None,
        })
        .collect();

    // Implement the chinese remainder theorem
    let modulo_product: usize = sequence.iter().map(|b| b.id).product();

    let time_congruent: usize = sequence
        .iter()
        .map(|b| {
            // This is a little confusing.
            // Suppose our bus_id=5, and our offset=1.
            // That means we want t + 1 to be divisible by 5.
            // To achieve this we will use chinese remainder theorem
            // to find t == 4 mod 5. (4 is *one* before our modulus of 5).
            // i.e., We calculate a remainder so that when we add the offset
            // to it, we are exactly divisible by the bus_id.
            let modulo = b.id;
            let rem = b.id - (b.offset % b.id);
            let n = modulo_product / modulo;
            let x = (1..=modulo)
                .find(|x| ((n * x) % modulo) == 1)
                .expect("To find x");
            rem * n * x
        })
        .sum();

    time_congruent % modulo_product
}

#[derive(Debug)]
struct BusInput {
    earliest_timestamp: usize,
    bus_ids: Vec<BusId>,
}

#[derive(Debug)]
enum BusId {
    Normal(usize),
    X,
}

fn inputs() -> BusInput {
    let text = crate::util::read_file("inputs/day13.txt");
    parse(&text)
}

fn parse(text: &str) -> BusInput {
    let mut lines = text.lines().filter_map(crate::util::not_blank);
    let earliest_timestamp = lines
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Valid timestamp");
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(map_to_bus_id)
        .collect();

    BusInput {
        earliest_timestamp,
        bus_ids,
    }
}

fn map_to_bus_id(s: &str) -> BusId {
    match s {
        "x" => BusId::X,
        num => BusId::Normal(num.parse().expect("Valid bus id")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"
            939
            7,13,x,x,59,x,31,19
        ";

        let input = parse(input);
        assert_eq!(295, calculate_part_1(&input));
        assert_eq!(1068781, calculate_part_2(&input));
    }

    fn run_part_2_test(s: &str) -> usize {
        let input = BusInput {
            earliest_timestamp: 1,
            bus_ids: s.split(',').map(map_to_bus_id).collect(),
        };
        calculate_part_2(&input)
    }

    #[test]
    fn extra_part_2_examples() {
        assert_eq!(3417, run_part_2_test("17,x,13,19"));
        assert_eq!(754018, run_part_2_test("67,7,59,61"));
        assert_eq!(779210, run_part_2_test("67,x,7,59,61"));
        assert_eq!(1261476, run_part_2_test("67,7,x,59,61"));
        assert_eq!(1202161486, run_part_2_test("1789,37,47,1889"));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(2845, calculate_part_1(&inputs));
        assert_eq!(487905974205117, calculate_part_2(&inputs));
    }
}
