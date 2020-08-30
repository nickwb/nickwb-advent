use regex::Regex;

type BaseInt = i32;

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

impl Moon {
    #[cfg(test)]
    fn from_coordinates(
        px: BaseInt,
        py: BaseInt,
        pz: BaseInt,
        vx: BaseInt,
        vy: BaseInt,
        vz: BaseInt,
    ) -> Moon {
        Moon {
            position: Vector3 {
                x: px,
                y: py,
                z: pz,
            },
            velocity: Vector3 {
                x: vx,
                y: vy,
                z: vz,
            },
        }
    }
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

    pub fn mut_refs(&mut self) -> (&mut Moon, &mut Moon, &mut Moon, &mut Moon) {
        unsafe {
            let ptr = self.moons.as_mut_ptr();
            let a = &mut *ptr;
            let b = &mut *ptr.add(1);
            let c = &mut *ptr.add(2);
            let d = &mut *ptr.add(3);
            (a, b, c, d)
        }
    }
}

fn calculate_part_one(moons: &MoonSet, total_steps: u64) -> (MoonSet, BaseInt) {
    let mut copy = moons.clone();
    simulate_motion(&mut copy, |step, _| step >= total_steps);
    let energy = copy.moons.iter().map(calculate_energy).sum();
    (copy, energy)
}

fn calculate_part_two(moons: &MoonSet) -> u64 {
    unimplemented!();
}

fn calculate_energy(moon: &Moon) -> BaseInt {
    let pot = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
    let kin = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
    pot * kin
}

fn simulate_motion<F: Fn(u64, &MoonSet) -> bool>(moons: &mut MoonSet, stop_fn: F) {
    fn gravity(a: &mut Moon, b: &mut Moon) {
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

    fn velocity(moon: &mut Moon) {
        moon.position.x += moon.velocity.x;
        moon.position.y += moon.velocity.y;
        moon.position.z += moon.velocity.z;
    }

    let mut step: u64 = 0;

    while !stop_fn(step, moons) {
        let (a, b, c, d) = moons.mut_refs();
        rayon::join(|| gravity(a, b), || gravity(c, d));
        rayon::join(|| gravity(a, c), || gravity(b, d));
        rayon::join(|| gravity(a, d), || gravity(b, c));

        rayon::join(|| velocity(a), || velocity(b));
        rayon::join(|| velocity(c), || velocity(d));

        step += 1;
    }
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

pub fn run_day_twelve() {
    let moons = inputs();
    println!("Day 12, Part 1: {}", calculate_part_one(&moons, 1000).1);
}

#[test]
fn example_1() {
    let input = r"
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    ";

    let moons = MoonSet::from_str(input);
    let result = calculate_part_one(&moons, 10);

    let expected = MoonSet {
        moons: [
            Moon::from_coordinates(2, 1, -3, -3, -2, 1),
            Moon::from_coordinates(1, -8, 0, -1, 1, 3),
            Moon::from_coordinates(3, -6, 1, 3, 2, -3),
            Moon::from_coordinates(2, 0, 4, 1, -1, -1),
        ],
    };

    assert_eq!(result.0, expected);
    assert_eq!(result.1, 179);
}

#[test]
fn example_2() {
    let input = r"
    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    ";

    let moons = MoonSet::from_str(input);
    let result = calculate_part_one(&moons, 100);

    let expected = MoonSet {
        moons: [
            Moon::from_coordinates(8, -12, -9, -7, 3, 0),
            Moon::from_coordinates(13, 16, -3, 3, -11, -5),
            Moon::from_coordinates(-29, -11, -1, -3, 7, 4),
            Moon::from_coordinates(16, -13, 23, 7, 1, 1),
        ],
    };

    assert_eq!(result.0, expected);
    assert_eq!(result.1, 1940);
}

#[test]
fn actual_part_1() {
    let moons = inputs();
    let result = calculate_part_one(&moons, 1000);
    assert_eq!(8287, result.1);
}
