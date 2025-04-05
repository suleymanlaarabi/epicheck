use std::thread::{JoinHandle, spawn};

use args::AppArgs;
use check_allowed::check_libc_usage;
use checker::print_file_error;
use clap::Parser;
use collect_files::collect_c_files;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use read_allowed::allowlist_as_str_slice;

mod args;
mod c_parsing;
mod check_allowed;
mod checker;
mod collect_files;
mod parser;
mod read_allowed;
mod rules;

fn main() {
    let args = AppArgs::parse();
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    if let Some(binary) = args.binary {
        threads.push(spawn(move || {
            let allowed_funcs = allowlist_as_str_slice("./allowlist.txt");
            check_libc_usage(&allowed_funcs, &binary);
        }));
    }

    collect_c_files(args.dirs)
        .par_iter()
        .for_each(|file| print_file_error(file.clone()));

    for th in threads {
        let _ = th.join();
    }
}
