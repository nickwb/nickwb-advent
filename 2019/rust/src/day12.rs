use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

const ZERO_VECTOR: Vector3 = Vector3 { x: 0, y: 0, z: 0 };

#[derive(Debug, PartialEq)]
struct Moon {
    pub position: Vector3,
    pub velocity: Vector3,
}

fn calculate_part_one(moons: &mut Vec<Moon>, total_steps: u64) -> i64 {
    simulate_motion(moons, total_steps);
    moons.iter().map(calculate_energy).sum()
}

fn calculate_energy(moon: &Moon) -> i64 {
    let pot = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
    let kin = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
    pot * kin
}

fn simulate_motion(moons: &mut Vec<Moon>, total_steps: u64) {
    let mut step: u64 = 0;
    while step < total_steps {
        with_pairs(moons, |a, b| {
            if a.position.x < b.position.x {
                a.velocity.x += 1;
                b.velocity.x -= 1;
            } else if a.position.x > b.position.x {
                a.velocity.x -= 1;
                b.velocity.x += 1;
            }

            if a.position.y < b.position.y {
                a.velocity.y += 1;
                b.velocity.y -= 1;
            } else if a.position.y > b.position.y {
                a.velocity.y -= 1;
                b.velocity.y += 1;
            }

            if a.position.z < b.position.z {
                a.velocity.z += 1;
                b.velocity.z -= 1;
            } else if a.position.z > b.position.z {
                a.velocity.z -= 1;
                b.velocity.z += 1;
            }
        });

        for m in moons.iter_mut() {
            m.position.x += m.velocity.x;
            m.position.y += m.velocity.y;
            m.position.z += m.velocity.z;
        }

        step += 1;
    }
}

fn with_pairs<F: Fn(&mut T, &mut T) -> (), T>(list: &mut Vec<T>, f: F) {
    for a in 0..list.len() {
        for b in (a + 1)..list.len() {
            let (a_split, b_split) = list.split_at_mut(b);
            let x = &mut a_split[a];
            let y = &mut b_split[0];
            f(x, y);
        }
    }
}

fn initial_moons(positions: Vec<Vector3>) -> Vec<Moon> {
    positions
        .iter()
        .map(|p| Moon {
            position: p.clone(),
            velocity: ZERO_VECTOR.clone(),
        })
        .collect()
}

fn inputs() -> Vec<Vector3> {
    let text = crate::util::read_file("inputs/day12.txt");
    parse_input(&text)
}

fn parse_input(text: &str) -> Vec<Vector3> {
    text.lines()
        .filter_map(|l| parse_vector(l.trim()))
        .collect()
}

lazy_static! {
    static ref VECTOR_PATTERN: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
}

fn parse_vector(text: &str) -> Option<Vector3> {
    let found = VECTOR_PATTERN.captures(text)?;
    let x = found.get(1)?.as_str().parse::<i64>().ok()?;
    let y = found.get(2)?.as_str().parse::<i64>().ok()?;
    let z = found.get(3)?.as_str().parse::<i64>().ok()?;
    Some(Vector3 { x, y, z })
}

pub fn run_day_twelve() {
    let mut moons = initial_moons(inputs());
    println!("Day 12, Part 1: {}", calculate_part_one(&mut moons, 1000));
}

#[test]
fn test_parse() {
    let input = r"
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    let parsed = parse_input(input);
    assert_eq!(4, parsed.len());
    assert_eq!(&Vector3 { x: -1, y: 0, z: 2 }, parsed.get(0).unwrap());
    assert_eq!(
        &Vector3 {
            x: 2,
            y: -10,
            z: -7
        },
        parsed.get(1).unwrap()
    );
    assert_eq!(&Vector3 { x: 4, y: -8, z: 8 }, parsed.get(2).unwrap());
    assert_eq!(&Vector3 { x: 3, y: 5, z: -1 }, parsed.get(3).unwrap());
}

#[test]
fn example_1() {
    let input = r"
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    ";

    let mut moons = initial_moons(parse_input(input));
    let energy = calculate_part_one(&mut moons, 10);

    let expected = vec![
        Moon {
            position: Vector3 { x: 2, y: 1, z: -3 },
            velocity: Vector3 { x: -3, y: -2, z: 1 },
        },
        Moon {
            position: Vector3 { x: 1, y: -8, z: 0 },
            velocity: Vector3 { x: -1, y: 1, z: 3 },
        },
        Moon {
            position: Vector3 { x: 3, y: -6, z: 1 },
            velocity: Vector3 { x: 3, y: 2, z: -3 },
        },
        Moon {
            position: Vector3 { x: 2, y: 0, z: 4 },
            velocity: Vector3 { x: 1, y: -1, z: -1 },
        },
    ];

    assert_eq!(moons, expected);
    assert_eq!(energy, 179);
}

#[test]
fn example_2() {
    let input = r"
    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    ";

    let mut moons = initial_moons(parse_input(input));
    let energy = calculate_part_one(&mut moons, 100);

    let expected = vec![
        Moon {
            position: Vector3 {
                x: 8,
                y: -12,
                z: -9,
            },
            velocity: Vector3 { x: -7, y: 3, z: 0 },
        },
        Moon {
            position: Vector3 {
                x: 13,
                y: 16,
                z: -3,
            },
            velocity: Vector3 {
                x: 3,
                y: -11,
                z: -5,
            },
        },
        Moon {
            position: Vector3 {
                x: -29,
                y: -11,
                z: -1,
            },
            velocity: Vector3 { x: -3, y: 7, z: 4 },
        },
        Moon {
            position: Vector3 {
                x: 16,
                y: -13,
                z: 23,
            },
            velocity: Vector3 { x: 7, y: 1, z: 1 },
        },
    ];

    assert_eq!(moons, expected);
    assert_eq!(energy, 1940);
}

#[test]
fn actual_part_1() {
    let mut moons = initial_moons(inputs());
    let energy = calculate_part_one(&mut moons, 1000);
    assert_eq!(8287, energy);
}
