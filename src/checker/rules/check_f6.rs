use crate::c_parsing::C_TYPE;

use super::c_error::C_F6;

pub fn check_f6(line: &str) -> Option<&'static str> {
    let mut containe_type = false;
    for c_type in C_TYPE {
        if line.contains(c_type) {
            containe_type = true;
        }
    }
    if !line.contains("=") && line.contains("()") && containe_type {
        return Some(C_F6);
    }
    None
}
