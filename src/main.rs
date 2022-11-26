extern crate bundler;
extern crate clap;

use clap::Parser;

/// Creates a single-source-file version of a Cargo package.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Which bin to expand
    #[arg(short, long)]
    bin: Option<String>,
    /// Cargo project path
    project: String,
}

fn main() {
    let cli = Cli::parse();
    let code = bundler::bundle(&cli.project, cli.bin.as_deref());
    println!("{}", code);
}
