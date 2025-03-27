use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn allowlist_as_str_slice(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to read allowlist.txt");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}
