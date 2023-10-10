use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of -inl.h files to merge into their parents
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for arg in args.files.iter() {
        println!("{}", arg);
    }
}
