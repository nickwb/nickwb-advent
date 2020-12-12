pub fn run_day_twelve() {
    let inputs = inputs();
    println!("Day 12, Part 1: {}", calculate_part_1(&inputs));
    println!("Day 12, Part 2: {}", calculate_part_2(&inputs));
}

type Position = (isize, isize);
type Step = (StepType, isize);

struct Instructions {
    steps: Vec<Step>,
}

#[derive(PartialEq)]
enum StepType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct StateP1 {
    position: Position,
    direction: isize,
}

struct StateP2 {
    position: Position,
    waypoint_relative: Position,
}

fn calculate_part_1(input: &Instructions) -> isize {
    let mut state = StateP1 {
        position: (0, 0),
        direction: 90,
    };

    for s in &input.steps {
        apply_step_part_1(&mut state, s);
    }

    manhattan(&state.position)
}

fn calculate_part_2(input: &Instructions) -> isize {
    let mut state = StateP2 {
        position: (0, 0),
        waypoint_relative: (10, 1),
    };

    for s in &input.steps {
        apply_step_part_2(&mut state, s);
    }

    manhattan(&state.position)
}

fn apply_step_part_1(state: &mut StateP1, step: &Step) {
    match step {
        (StepType::North, v) => {
            state.position.1 += v;
        }
        (StepType::South, v) => {
            state.position.1 -= v;
        }
        (StepType::East, v) => {
            state.position.0 += v;
        }
        (StepType::West, v) => {
            state.position.0 -= v;
        }
        (StepType::Left, v) => {
            state.direction = normalize_direction(state.direction - v);
        }
        (StepType::Right, v) => {
            state.direction = normalize_direction(state.direction + v);
        }
        (StepType::Forward, v) => match state.direction {
            0 => apply_step_part_1(state, &(StepType::North, *v)),
            90 => apply_step_part_1(state, &(StepType::East, *v)),
            180 => apply_step_part_1(state, &(StepType::South, *v)),
            270 => apply_step_part_1(state, &(StepType::West, *v)),
            _ => panic!("Don't know how to do that"),
        },
    }
}

fn apply_step_part_2(state: &mut StateP2, step: &Step) {
    match step {
        (StepType::North, v) => {
            state.waypoint_relative.1 += v;
        }
        (StepType::South, v) => {
            state.waypoint_relative.1 -= v;
        }
        (StepType::East, v) => {
            state.waypoint_relative.0 += v;
        }
        (StepType::West, v) => {
            state.waypoint_relative.0 -= v;
        }
        (StepType::Left, v) => {
            match v % 360 {
                0 => rotate(state, (1, 0)),
                90 => rotate(state, (0, 1)),
                180 => rotate(state, (-1, 0)),
                270 => rotate(state, (0, -1)),
                _ => panic!("Don't know how to do that"),
            };
        }
        (StepType::Right, v) => {
            match v % 360 {
                0 => rotate(state, (1, 0)),
                90 => rotate(state, (0, -1)),
                180 => rotate(state, (-1, 0)),
                270 => rotate(state, (0, 1)),
                _ => panic!("Don't know how to do that"),
            };
        }
        (StepType::Forward, v) => {
            let step = (state.waypoint_relative.0 * v, state.waypoint_relative.1 * v);
            state.position = (state.position.0 + step.0, state.position.1 + step.1);
        }
    }
}

fn normalize_direction(mut d: isize) -> isize {
    while d < 0 {
        d += 360;
    }

    while d >= 360 {
        d -= 360;
    }

    d
}

fn rotate(state: &mut StateP2, cos_sin_t: (isize, isize)) {
    let (cos_t, sin_t) = cos_sin_t;
    let new_relative = (
        (state.waypoint_relative.0 * cos_t) - (state.waypoint_relative.1 * sin_t),
        (state.waypoint_relative.0 * sin_t) + (state.waypoint_relative.1 * cos_t),
    );
    state.waypoint_relative = new_relative;
}

fn manhattan(position: &Position) -> isize {
    position.0.abs() + position.1.abs()
}

fn inputs() -> Instructions {
    let text = crate::util::read_file("inputs/day12.txt");
    parse(&text)
}

fn parse(text: &str) -> Instructions {
    let steps = text
        .lines()
        .filter_map(crate::util::not_blank)
        .map(|l| {
            let step_type = match l.chars().next().unwrap() {
                'N' => StepType::North,
                'S' => StepType::South,
                'E' => StepType::East,
                'W' => StepType::West,
                'L' => StepType::Left,
                'R' => StepType::Right,
                'F' => StepType::Forward,
                _ => panic!("Unexpected step type"),
            };

            let amount = (&l[1..]).parse::<usize>().expect("Invalid amount");

            if (step_type == StepType::Left || step_type == StepType::Right) && (amount % 90 != 0) {
                panic!("Unsupported turn angle");
            }

            (step_type, amount as isize)
        })
        .collect();

    Instructions { steps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
            F10
            N3
            F7
            R90
            F11
        ";

        let instructions = parse(text);
        assert_eq!(25, calculate_part_1(&instructions));
        assert_eq!(286, calculate_part_2(&instructions));
    }

    #[test]
    fn actual_inputs() {
        let inputs = inputs();
        assert_eq!(845, calculate_part_1(&inputs));
        assert_eq!(27016, calculate_part_2(&inputs));
    }
}
