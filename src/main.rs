use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the file to read, with extension (e.g. `file.law`)
    script: String,

    /// The path to the directory to read sub-routine files from
    #[arg(short, long)]
    dir_path: String,
}

fn main() {
    let args = Args::parse();
}
