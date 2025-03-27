use std::collections::HashSet;
use std::process::Command;

fn get_libc_functions(binary: &str) -> HashSet<String> {
    let output = Command::new("objdump")
        .args(["-T", binary])
        .output()
        .expect("Unabl to execute objdump check if is installed !");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .filter(|line| line.contains("GLIBC_"))
        .filter_map(|line| line.split_whitespace().last())
        .map(|s| s.to_string())
        .collect()
}

pub fn check_libc_usage(allowed_funcs: &Vec<String>, binary_path: &str) {
    let mut allowed: HashSet<String> = allowed_funcs.iter().map(|s| s.to_string()).collect();
    let always_allowed = [
        "__libc_start_main",
        "__cxa_finalize",
        "__environ",
        "__stack_chk_fail",
    ];
    allowed.extend(always_allowed.iter().map(|s| s.to_string()));

    let used_funcs = get_libc_functions(binary_path);

    let unauthorized: Vec<_> = used_funcs.difference(&allowed).cloned().collect();

    if !unauthorized.is_empty() {
        println!("Libc function unauthorized !:");
        let mut sorted = unauthorized;
        sorted.sort();
        for func in sorted {
            println!(" - {}", func);
        }
    } else {
        println!("All used function is authorized !");
    }
}
