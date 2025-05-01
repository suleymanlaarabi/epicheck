use super::c_error::C_G1;

pub fn check_g1(content: &str) -> Option<&'static str> {
    if !content.starts_with("/*\n** ") {
        return Some(C_G1);
    }
    None
}
