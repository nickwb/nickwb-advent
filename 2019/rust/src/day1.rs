fn get_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

fn get_total_fuel(module_mass: f64) -> f64 {
    let mut total: f64 = 0.0;
    let mut mass: f64 = get_fuel(module_mass);
    while mass > 0.0 {
        total = total + mass;
        mass = get_fuel(mass);
    }

    total
}

const MY_INPUTS: [i64; 100] = [
    137139, 104321, 137149, 82531, 97698, 56831, 115133, 64329, 111730, 145953, 73388, 57230,
    61935, 58542, 147631, 79366, 115484, 86997, 80362, 129109, 58568, 121969, 63696, 68116, 86668,
    62059, 59485, 132507, 107823, 94467, 53032, 140962, 129499, 80599, 147570, 96463, 126169,
    108575, 133312, 146929, 79826, 114449, 110908, 66107, 62171, 91677, 128188, 90483, 81045,
    96006, 110366, 140765, 148360, 54565, 56664, 121547, 78839, 123739, 115408, 123245, 92419,
    132564, 80022, 103370, 145366, 145211, 110360, 145897, 140817, 77978, 138064, 148134, 86208,
    89472, 67117, 63423, 148536, 105835, 107783, 98601, 66702, 50459, 55127, 79808, 79628, 76264,
    134392, 125547, 118186, 80947, 121669, 107315, 145093, 56296, 117226, 105409, 149238, 142651,
    103286, 139215,
];

pub fn run_day_one() {
    let part_one: f64 = MY_INPUTS.iter().map(|i| get_fuel(*i as f64)).sum();
    println!("Day 1, Part 1: {}", part_one);

    let part_two: f64 = MY_INPUTS.iter().map(|i| get_total_fuel(*i as f64)).sum();
    println!("Day 1, Part 2: {}", part_two);
}

#[test]
fn example_1() {
    assert_eq!(2.0, get_fuel(12.0));
}

#[test]
fn example_2() {
    assert_eq!(2.0, get_fuel(14.0));
}

#[test]
fn example_3() {
    assert_eq!(654.0, get_fuel(1969.0));
}

#[test]
fn example_4() {
    assert_eq!(33583.0, get_fuel(100756.0));
}

#[test]
fn example_5() {
    assert_eq!(2.0, get_total_fuel(14.0));
}

#[test]
fn example_6() {
    assert_eq!(966.0, get_total_fuel(1969.0));
}

#[test]
fn example_7() {
    assert_eq!(50346.0, get_total_fuel(100756.0));
}
