pub fn check_l3(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let mut count = 0;

    if line.starts_with("/") || line.starts_with("**") {
        return None;
    }

    for c in trimmed.chars() {
        if c.is_whitespace() {
            count += 1;
        } else {
            count = 0;
        }
        if count > 1 {
            return Some("C-L3".to_string());
        }
    }

    None
}
