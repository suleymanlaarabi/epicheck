pub fn check_f3(line: &str) -> Option<String> {
    if line.len() > 79 {
        return Some("C-F3".to_string());
    }
    return None;
}
