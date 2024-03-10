use std::process;
mod parsing;
mod processing;
use crate::parsing::csv_from_file;
use clap::Parser;

fn main() {
    let args = crate::parsing::Args::parse();
    println!("{:?}", args);
    if let Err(err) = csv_from_file(&args.path) {
        println!("{}", err);
        process::exit(1);
    }
}
