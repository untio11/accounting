mod parsing;
mod processing;
use crate::parsing::csv_from_path;
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = crate::parsing::Args::parse();
    println!("{:?}", args);
    if let Ok(transactions) = csv_from_path(&args.path) {
        println!("{:?}", transactions);
        return Ok(());
    }
    panic!("Couldn't parse transactions successfully.");
}
