pub fn check_g7(line: &str) -> Option<String> {
    let last_char = match line.chars().last() {
        Some(c) => c,
        None => return None,
    };

    if last_char == ' ' {
        return Some("C-G7".to_string());
    }
    return None;
}
