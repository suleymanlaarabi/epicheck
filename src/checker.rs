use std::fs::read_to_string;

use crate::rules::*;

pub struct CodingStyleError {
    pub name: String,
    pub line: usize,
}

impl CodingStyleError {
    pub fn print_coconut(&self, file_path: &str) {
        println!("{}:{}: MAJOR:{}", file_path, self.line, self.name);
    }
}

pub struct FileCodingStyleReport {
    pub file_path: String,
    pub errors: Vec<CodingStyleError>,
}

pub fn coding_style_report_from_file(file_path: String) -> FileCodingStyleReport {
    let mut errors: Vec<CodingStyleError> = Vec::new();
    let content =
        read_to_string(&file_path).expect(&format!("Unable to read the file: {}", &file_path));
    let mut line_count = 1;

    let mut f4_checker = F4Checker::default();

    for line in content.lines() {
        check_all_f4(&mut f4_checker, line);
        if let Some(name) = check_l2(line) {
            errors.push(CodingStyleError {
                name,
                line: line_count,
            });
        }
        if let Some(name) = check_f3(line) {
            errors.push(CodingStyleError {
                name,
                line: line_count,
            });
        }
        if let Some(name) = check_l3(line) {
            errors.push(CodingStyleError {
                name,
                line: line_count,
            });
        }
        if let Some(name) = check_l2(line) {
            errors.push(CodingStyleError {
                name,
                line: line_count,
            });
        }
        if let Some(name) = check_g7(line) {
            errors.push(CodingStyleError {
                name,
                line: line_count,
            });
        }
        line_count += 1;
    }
    for f4 in f4_checker.all_f4 {
        errors.push(CodingStyleError {
            name: "C-F4".to_string(),
            line: f4,
        });
    }
    FileCodingStyleReport { file_path, errors }
}

pub fn print_file_error(file: String) {
    let errors = coding_style_report_from_file(file);

    for error in errors.errors {
        error.print_coconut(&errors.file_path);
    }
}
