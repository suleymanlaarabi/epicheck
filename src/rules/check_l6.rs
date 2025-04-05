use super::c_error::C_L6;

pub fn check_l6(line: &str) -> Option<&'static str> {
    if line.contains(";;") {
        return Some(C_L6);
    }
    None
}
