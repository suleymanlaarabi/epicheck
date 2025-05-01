#[derive(Default)]
pub struct F4Checker {
    pub all_f4: Vec<usize>,
    current_func_lines: Vec<usize>,
    depth: isize,
    line_number: usize,
}

#[inline]
pub fn check_all_f4(f4_checker: &mut F4Checker, line: &str) {
    let (mut contains_open, mut code_has_content, mut delta) = (false, false, 0);
    for ch in line.chars() {
        match ch {
            '{' => {
                contains_open = true;
                delta += 1;
            }
            '}' => {
                delta -= 1;
            }
            _ if !ch.is_whitespace() => {
                code_has_content = true;
            }
            _ => {}
        }
    }

    if f4_checker.depth >= 1 && !contains_open {
        f4_checker.current_func_lines.push(f4_checker.line_number);
    } else if f4_checker.depth == 0 && contains_open && code_has_content {
        f4_checker.current_func_lines.push(f4_checker.line_number);
    }

    f4_checker.depth += delta;
    if f4_checker.depth < 0 {
        f4_checker.depth = 0;
    }

    if f4_checker.depth == 0 && !f4_checker.current_func_lines.is_empty() {
        if f4_checker.current_func_lines.len() > 20 {
            f4_checker
                .all_f4
                .extend(f4_checker.current_func_lines.iter().skip(20).cloned());
        }
        f4_checker.current_func_lines.clear();
    }

    f4_checker.line_number += 1;
}
