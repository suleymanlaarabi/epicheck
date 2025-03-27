use args::AppArgs;
use checker::print_file_error;
use clap::Parser;
use collect_files::collect_c_files;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod args;
mod checker;
mod collect_files;
mod rules;

fn main() {
    let args = AppArgs::parse();

    collect_c_files(args.dirs)
        .par_iter()
        .for_each(|file| print_file_error(file.clone()));
}
