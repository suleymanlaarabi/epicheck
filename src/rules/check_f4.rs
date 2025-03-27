pub fn check_f4(content: &str) -> Vec<usize> {
    let mut errors = Vec::new();
    let mut current_func_lines = Vec::new();
    let mut depth: isize = 0;
    let mut line_number = 0;

    for line in content.lines() {
        if depth >= 1 && !line.contains('{') {
            current_func_lines.push(line_number);
        } else if depth == 0 && line.contains('{') {
            let code: String = line.chars().filter(|&ch| ch != '{' && ch != '}').collect();
            if !code.trim().is_empty() {
                current_func_lines.push(line_number);
            }
        }
        for ch in line.chars() {
            match ch {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }
        if depth < 0 {
            depth = 0;
        }
        if depth == 0 && !current_func_lines.is_empty() {
            if current_func_lines.len() > 20 {
                errors.extend(current_func_lines.iter().skip(20).cloned());
            }
            current_func_lines.clear();
        }
        line_number += 1;
    }
    errors
}
