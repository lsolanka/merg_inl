use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use faccess::PathExt;
use merge_inl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of -inl.h files to merge into their parents
    files: Vec<PathBuf>,
}

fn check_if_args_exist(file_paths: &Vec<PathBuf>) -> Result<(), io::Error> {
    let mut bad_files: Vec<&Path> = vec![];

    for file_path in file_paths.iter() {
        if !(file_path.is_file() && file_path.readable()) {
            bad_files.push(file_path);
        }
    }

    if !bad_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            String::from("The following files are not readable:\n")
                + &file_list_to_string(&bad_files),
        ));
    }

    Ok(())
}

fn file_list_to_string(files: &Vec<&Path>) -> String {
    let mut file_string = String::new();

    for file in files.iter() {
        file_string += &format!("{}\n", file.to_string_lossy());
    }

    file_string
}

pub fn merge_inl_files(inl_files: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if let Err(error) = check_if_args_exist(inl_files) {
        eprint!("{}", error.to_string());
        process::exit(1);
    }

    for inl_file in inl_files.iter() {
        if let Some(parent_path) = merge_inl::get_parent_file_path(inl_file) {
            println!(
                "{} parent: {}",
                inl_file.display(),
                parent_path.to_str().unwrap()
            );
        } else {
            eprintln!(
                "{} is not a file with `-inl.h` or `_inl.h` suffix; skipping",
                inl_file.display()
            );
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    merge_inl_files(&args.files)
}
