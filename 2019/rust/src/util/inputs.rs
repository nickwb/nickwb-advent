use std::str::FromStr;

pub fn read_int_array<T: FromStr>(path: &str) -> Vec<T> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|l| l.split(','))
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| match v.parse::<T>() {
            Ok(v) => v,
            Err(_) => {
                panic!("read_int_array: Failed to parse a value");
            }
        })
        .collect()
}
