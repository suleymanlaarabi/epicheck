use super::c_error::C_G7;

pub fn check_g7(line: &str) -> Option<&'static str> {
    let last_char = match line.chars().last() {
        Some(c) => c,
        None => return None,
    };

    if last_char == ' ' {
        return Some(C_G7);
    }
    return None;
}
