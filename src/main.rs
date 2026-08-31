mod args;

use args::Cli;
use clap::Parser;

fn main() {
    let args : Cli = Cli::parse();
    println!("{:?}", args);
}
