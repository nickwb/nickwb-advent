use num::integer::lcm;
use regex::Regex;
use std::collections::HashSet;

type BaseInt = i32;

pub fn run_day_twelve() {
    let mut moons_one = inputs();
    let mut moons_two = moons_one.clone();
    println!(
        "Day 12, Part 1: {}",
        calculate_part_one(&mut moons_one, 1000)
    );
    println!("Day 12, Part 2: {}", calculate_part_two(&mut moons_two));
}

fn calculate_part_one(set: &mut MoonSet, total_steps: u64) -> BaseInt {
    simulate_motion(set, |step, _| step == total_steps);
    set.moons.iter().map(calculate_energy).sum()
}

fn calculate_part_two(set: &mut MoonSet) -> u64 {
    let mut x_states: HashSet<AxisState> = HashSet::new();
    let mut y_states: HashSet<AxisState> = HashSet::new();
    let mut z_states: HashSet<AxisState> = HashSet::new();
    let mut x_solution: Option<u64> = None;
    let mut y_solution: Option<u64> = None;
    let mut z_solution: Option<u64> = None;

    simulate_motion(set, |step_num, moons| {
        if x_solution.is_none() {
            let x = moons.get_axis(Axis::X);
            if x_states.contains(&x) {
                //eprintln!("Found a cycle in X after {}", step_num);
                x_solution = Some(step_num);
                x_states.clear();
            } else {
                x_states.insert(x);
            }
        }

        if y_solution.is_none() {
            let y = moons.get_axis(Axis::Y);
            if y_states.contains(&y) {
                //eprintln!("Found a cycle in Y after {}", step_num);
                y_solution = Some(step_num);
                y_states.clear();
            } else {
                y_states.insert(y);
            }
        }

        if z_solution.is_none() {
            let z = moons.get_axis(Axis::Z);
            if z_states.contains(&z) {
                //eprintln!("Found a cycle in Z after {}", step_num);
                z_solution = Some(step_num);
                z_states.clear();
            } else {
                z_states.insert(z);
            }
        }

        x_solution.is_some() && y_solution.is_some() && z_solution.is_some()
    });

    let a = lcm(x_solution.unwrap(), y_solution.unwrap());
    lcm(a, z_solution.unwrap())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct AxisState {
    positions: [BaseInt; 4],
    velocities: [BaseInt; 4],
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector3 {
    pub x: BaseInt,
    pub y: BaseInt,
    pub z: BaseInt,
}

const ZERO_VECTOR: Vector3 = Vector3 { x: 0, y: 0, z: 0 };

#[derive(Debug, PartialEq, Clone, Copy)]
struct Moon {
    pub position: Vector3,
    pub velocity: Vector3,
}

const ZERO_MOON: Moon = Moon {
    position: ZERO_VECTOR,
    velocity: ZERO_VECTOR,
};

#[derive(Debug, Clone, PartialEq)]
struct MoonSet {
    pub moons: [Moon; 4],
}

impl MoonSet {
    fn from_str(spec: &str) -> MoonSet {
        let vectors = spec.lines().filter_map(|l| parse_vector(l.trim()));
        let mut idx = 0;
        let mut moons: [Moon; 4] = [ZERO_MOON; 4];
        for position in vectors {
            if idx > 3 {
                panic!("Too many moons");
            }
            moons[idx].position = position;
            idx += 1;
        }

        if idx != 4 {
            panic!("Not enough moons");
        }

        MoonSet { moons }
    }

    fn mut_refs(&mut self) -> (&mut Moon, &mut Moon, &mut Moon, &mut Moon) {
        unsafe {
            let ptr = self.moons.as_mut_ptr();
            let a = &mut *ptr;
            let b = &mut *ptr.add(1);
            let c = &mut *ptr.add(2);
            let d = &mut *ptr.add(3);
            (a, b, c, d)
        }
    }

    fn get_axis(&self, axis: Axis) -> AxisState {
        let mut positions = [0; 4];
        let mut velocities = [0; 4];
        for (i, m) in self.moons.iter().enumerate() {
            match axis {
                Axis::X => {
                    positions[i] = m.position.x;
                    velocities[i] = m.velocity.x;
                }
                Axis::Y => {
                    positions[i] = m.position.y;
                    velocities[i] = m.velocity.y;
                }
                Axis::Z => {
                    positions[i] = m.position.z;
                    velocities[i] = m.velocity.z;
                }
            }
        }

        AxisState {
            positions,
            velocities,
        }
    }
}

fn calculate_energy(moon: &Moon) -> BaseInt {
    let pot = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
    let kin = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
    pot * kin
}

fn simulate_motion<F: FnMut(u64, &MoonSet) -> bool>(moons: &mut MoonSet, mut stop_fn: F) -> u64 {
    fn apply_gravity(a: &mut Moon, b: &mut Moon) {
        let x = (b.position.x - a.position.x).signum();
        let y = (b.position.y - a.position.y).signum();
        let z = (b.position.z - a.position.z).signum();

        a.velocity.x += x;
        a.velocity.y += y;
        a.velocity.z += z;
        b.velocity.x -= x;
        b.velocity.y -= y;
        b.velocity.z -= z;
    }

    fn apply_velocity(moon: &mut Moon) {
        moon.position.x += moon.velocity.x;
        moon.position.y += moon.velocity.y;
        moon.position.z += moon.velocity.z;
    }

    let mut step: u64 = 0;

    while !stop_fn(step, moons) {
        let (a, b, c, d) = moons.mut_refs();
        apply_gravity(a, b);
        apply_gravity(c, d);
        apply_gravity(a, c);
        apply_gravity(b, d);
        apply_gravity(a, d);
        apply_gravity(b, c);
        apply_velocity(a);
        apply_velocity(b);
        apply_velocity(c);
        apply_velocity(d);

        step += 1;
    }

    step
}

fn inputs() -> MoonSet {
    let text = crate::util::read_file("inputs/day12.txt");
    MoonSet::from_str(&text)
}

lazy_static! {
    static ref VECTOR_PATTERN: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
}

fn parse_vector(text: &str) -> Option<Vector3> {
    let found = VECTOR_PATTERN.captures(text)?;
    let x = found.get(1)?.as_str().parse::<BaseInt>().ok()?;
    let y = found.get(2)?.as_str().parse::<BaseInt>().ok()?;
    let z = found.get(3)?.as_str().parse::<BaseInt>().ok()?;
    Some(Vector3 { x, y, z })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>
        ";

        let mut moons_one = MoonSet::from_str(input);
        let mut moons_two = moons_one.clone();
        assert_eq!(179, calculate_part_one(&mut moons_one, 10));
        assert_eq!(2772, calculate_part_two(&mut moons_two));
    }

    #[test]
    fn example_2() {
        let input = r"
            <x=-8, y=-10, z=0>
            <x=5, y=5, z=10>
            <x=2, y=-7, z=3>
            <x=9, y=-8, z=-3>
        ";

        let mut moons_one = MoonSet::from_str(input);
        let mut moons_two = moons_one.clone();
        assert_eq!(1940, calculate_part_one(&mut moons_one, 100));
        assert_eq!(4686774924, calculate_part_two(&mut moons_two));
    }

    #[test]
    fn actual_part_1() {
        let mut moons_one = inputs();
        let mut moons_two = moons_one.clone();
        assert_eq!(8287, calculate_part_one(&mut moons_one, 1000));
        assert_eq!(528250271633772, calculate_part_two(&mut moons_two));
    }
}
