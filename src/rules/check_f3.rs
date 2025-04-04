use super::c_error::C_F3;

pub fn check_f3(line: &str) -> Option<&'static str> {
    if line.len() > 79 {
        return Some(C_F3);
    }
    return None;
}
