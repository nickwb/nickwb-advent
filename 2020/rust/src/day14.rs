use regex::Regex;
use std::collections::HashMap;

pub fn run_day_fourteen() {
    let inputs = inputs();
    println!("Day 14, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 14, Part 2: {}", calculate_part_2(&inputs));
}

fn calculate_part_1(input: &MaskInput) -> usize {
    emulate_returning_sum(input, |mem, mask, v| {
        let value = (v.value & !mask.fixed_mask) | mask.fixed_values;
        mem.insert(v.address, value);
    })
}

fn calculate_part_2(input: &MaskInput) -> usize {
    emulate_returning_sum(input, |mem, mask, v| {
        let base_addr = v.address | mask.fixed_values;
        permute_bits_recursive(mask, base_addr, 0, mem, v.value);
    })
}

fn permute_bits_recursive(
    mask: &Mask,
    base_addr: usize,
    bit_idx: usize,
    mem: &mut Mem,
    value: usize,
) {
    // If bit_idx is 36, we'll skip the loop and immediately
    // write to memory.
    for idx in bit_idx..36 {
        let bit = 1 << idx;
        if mask.fixed_mask & bit > 0 {
            // Fixed bit, we don't need to permute it
            continue;
        }

        let one = base_addr | bit;
        let zero = base_addr & !bit;
        permute_bits_recursive(mask, one, idx + 1, mem, value);
        permute_bits_recursive(mask, zero, idx + 1, mem, value);

        // Let the recursive calls write to memory.
        // We've done our job
        return;
    }

    // Once we reach bit 36, we can actually write to memory,
    // there's no further permutations
    mem.insert(base_addr, value);
}

type Mem = HashMap<usize, usize>;

fn emulate_returning_sum<F: FnMut(&mut Mem, &Mask, &MemValue) -> ()>(
    input: &MaskInput,
    mut write_fn: F,
) -> usize {
    let mut mask = Mask {
        fixed_values: 0,
        fixed_mask: 0,
    };

    let mut memory: Mem = HashMap::new();

    for i in input.instructions.iter() {
        match i {
            Instruction::SetMask(m) => {
                mask = m.clone();
            }
            Instruction::WriteMem(v) => {
                write_fn(&mut memory, &mask, &v);
            }
        }
    }

    memory.values().sum()
}

#[derive(Debug)]
struct MaskInput {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    SetMask(Mask),
    WriteMem(MemValue),
}

#[derive(Debug, Clone)]
struct Mask {
    fixed_values: usize,
    fixed_mask: usize,
}

#[derive(Debug)]
struct MemValue {
    address: usize,
    value: usize,
}

fn inputs() -> MaskInput {
    let text = crate::util::read_file("inputs/day14.txt");
    parse(&text)
}

lazy_static! {
    static ref MASK_PATTERN: Regex = Regex::new(r"^mask = ([X10]+)$").unwrap();
    static ref MEM_PATTERN: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

fn parse(text: &str) -> MaskInput {
    let instructions = text
        .lines()
        .filter_map(crate::util::not_blank)
        .map(|l| {
            // Is it m[a]sk or m[e]m ?
            if &l[1..2] == "a" {
                let captures = MASK_PATTERN.captures(l).unwrap();
                let (fixed_values, fixed_mask) =
                    captures[1]
                        .chars()
                        .fold((0, 0), |(values, mask), c| match c {
                            'X' => (values << 1, mask << 1),
                            '0' => (values << 1, (mask << 1) | 1),
                            '1' => ((values << 1) | 1, (mask << 1) | 1),
                            _ => panic!("Unexpected char"),
                        });
                Instruction::SetMask(Mask {
                    fixed_values,
                    fixed_mask,
                })
            } else {
                let captures = MEM_PATTERN.captures(l).unwrap();
                Instruction::WriteMem(MemValue {
                    address: captures[1].parse().expect("Valid address"),
                    value: captures[2].parse().expect("Valid value"),
                })
            }
        })
        .collect();

    MaskInput { instructions }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        ";

        assert_eq!(165, calculate_part_1(&parse(text)));
    }

    #[test]
    fn example_2() {
        let text = r"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        ";

        assert_eq!(208, calculate_part_2(&parse(text)));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(6317049172545, calculate_part_1(&inputs));
        assert_eq!(3434009980379, calculate_part_2(&inputs));
    }
}
