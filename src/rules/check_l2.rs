use super::c_error::C_L2;

pub fn check_l2(line: &str) -> Option<&'static str> {
    let mut count = 0;
    for c in line.chars() {
        if c == ' ' {
            count += 1;
        } else {
            break;
        }
    }
    if count != 0 && count % 4 != 0 {
        return Some(C_L2);
    }
    return None;
}
