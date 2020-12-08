pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn not_blank(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None 
    } else {
        Some(trimmed)
    }
}