use super::c_error::C_L3;

pub fn check_l3(line: &str) -> Option<&'static str> {
    let trimmed = line.trim();
    let mut count = 0;
    let mut prev = '\0';

    for c in trimmed.chars() {
        if c.is_whitespace() {
            count += 1;
        } else {
            count = 0;
        }
        if count > 1 {
            return Some(C_L3);
        }
        if prev == ' ' && c == ';' {
            return Some(C_L3);
        }
        prev = c;
    }

    None
}
