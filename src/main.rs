use clap::Parser;
use merge_inl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of -inl.h files to merge into their parents
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for arg in args.files.iter() {
        if let Some(parent_path) = merge_inl::get_parent_file_path(&arg) {
            println!("{} parent: {}", arg, parent_path.to_str().unwrap());
        } else {
            eprintln!("{} is not a file with `-inl.h` suffix; skipping", arg);
        }
    }
}
