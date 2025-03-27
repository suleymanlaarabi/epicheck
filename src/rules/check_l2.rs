pub fn check_l2(line: &str) -> Option<String> {
    let mut count = 0;
    for c in line.chars() {
        if c == ' ' {
            count += 1;
        } else {
            break;
        }
    }
    if count != 0 && count % 4 != 0 {
        return Some("C-L2".to_string());
    }
    return None;
}
