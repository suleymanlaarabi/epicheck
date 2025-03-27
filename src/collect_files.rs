use std::fs;

pub fn collect_c_files(mut stack: Vec<String>) -> Vec<String> {
    let mut entries = Vec::new();

    while let Some(path) = stack.pop() {
        if let Ok(read_dir) = fs::read_dir(&path) {
            for entry in read_dir.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.to_string_lossy().into_owned());
                } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.len() > 2 && name.ends_with(".c") {
                        entries.push(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    entries
}
