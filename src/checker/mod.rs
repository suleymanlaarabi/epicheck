use std::{fs::read_to_string, process::exit};

pub mod checker_ctx;

mod rules;
use rules::{c_error::C_F4, *};

pub struct CodingStyleError {
    pub name: &'static str,
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

type Func = fn(&str) -> Option<&'static str>;

const FUNCS: [Func; 6] = [check_l6, check_f6, check_f3, check_g7, check_l3, check_l2];

pub fn push_error_if(
    func: Func,
    content: &str,
    line: usize,
    errors: &mut Vec<CodingStyleError>,
) -> bool {
    if let Some(name) = func(content) {
        errors.push(CodingStyleError { name, line });
        return true;
    }
    return false;
}

pub fn coding_style_report_from_file(file_path: String) -> FileCodingStyleReport {
    let mut errors: Vec<CodingStyleError> = Vec::new();
    let content = match read_to_string(&file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Invalide C file: {}", file_path);
            exit(1);
        }
    };

    let mut line_count = 0;
    let mut f4_checker = F4Checker::default();

    push_error_if(check_g1, &content, 1, &mut errors);
    for line in content.lines() {
        line_count += 1;
        if line.starts_with("/") || line.starts_with("**") {
            continue;
        }
        for func in FUNCS {
            if push_error_if(func, line, line_count, &mut errors) {
                break;
            }
        }
        check_all_f4(&mut f4_checker, line);
    }
    for f4 in f4_checker.all_f4 {
        errors.push(CodingStyleError {
            name: C_F4,
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
