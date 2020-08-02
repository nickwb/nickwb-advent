use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

const ZERO_VECTOR: Vector3 = Vector3 { x: 0, y: 0, z: 0 };

#[derive(Debug)]
struct Moon {
    pub position: Vector3,
    pub velocity: Vector3,
}

fn calculate_part_one(mut moons: Vec<Moon>, total_steps: u64) -> i64 {
    let mut step: u64 = 0;
    while step < total_steps {
        with_pairs(&mut moons, |a, b| {
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

    unimplemented!()
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
    println!("TODO");
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
