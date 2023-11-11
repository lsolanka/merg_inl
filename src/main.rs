use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use merge_inl;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of -inl.h files to merge into their parents
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if let Err(error) = merge_inl::merge(&args.files) {
        eprintln!("{}", error);
    }

    Ok(())
}
