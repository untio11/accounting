mod parsing;
mod processing;
use crate::{parsing::transactions_from_path, processing::summaries};
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = crate::parsing::Args::parse();
    println!("{:?}", args);
    if let Ok(transactions) = transactions_from_path(&args.path) {
        let node_freq = summaries::node_frequencies(transactions);
        println!("Node frequencies: {:?}", node_freq);

        return Ok(());
    }
    panic!("Couldn't parse transactions successfully.");
}
