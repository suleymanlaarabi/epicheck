use crate::c_parsing::C_TYPE;

pub fn check_f6(line: &str) -> Option<String> {
    let mut containe_type = false;
    for c_type in C_TYPE {
        if line.contains(c_type) {
            containe_type = true;
        }
    }
    if containe_type && line.contains("()") && !line.contains("=") {
        return Some("C-F6".to_string());
    }
    None
}
