use regex::Regex;

lazy_static! {
    static ref VECTOR_PATTERN: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
}

#[derive(Debug, PartialEq)]
struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

fn parse_input(text: &str) -> Vec<Vector3> {
    text.lines()
        .filter_map(|l| parse_vector(l.trim()))
        .collect()
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
