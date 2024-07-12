mod parsing;
mod processing;
mod state;
use crate::{parsing::transactions_from_path, processing::summaries};
use clap::Parser;
use color_eyre::Result;
use itertools::Itertools;
use parsing::{profile_from_path, Direction};
use processing::{
    types::{Node, Transaction},
    Identify,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = crate::parsing::Args::parse();
    println!("{:?}", args);

    let me = profile_from_path(&args.profile_path);
    if let Ok(transactions) = transactions_from_path(&args.csv_path) {
        println!("Accessing first 5 elements:");
        for line in &transactions.data()[..5] {
            print_csv_line(line, me.owns.first().unwrap());
        }

        let node_freq = summaries::node_frequencies(&transactions);
        let node_freq: Vec<_> = node_freq.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect();

        for (id, count) in node_freq.iter() {
            println!(
                "{:?}: {count}",
                match me.view(id) {
                    Some(owned_node) => owned_node.to_string(),
                    None => id.to_string(),
                }
            );
        }

        println!(
            "Date range: {:?}",
            [transactions.data().first(), transactions.data().last()].map(|t| t.unwrap().date)
        );

        return Ok(());
    }
    panic!("Couldn't parse transactions successfully.");
}

fn print_csv_line(line: &Transaction, perspective: &Node) {
    println!("\n+==================+");
    println!(
        "| {} | ({})",
        line.id(),
        if line.direction(perspective) == Direction::Incoming {
            "+"
        } else {
            "-"
        }
    );
    println!("+==================+");
    println!("| Amount:      {}", line.amount);
    println!("| Date:        {}", line.date);
    println!("| Source:      {}", line.source);
    println!("| Sink:        {}", line.sink);
    println!("| Inhrnt Tgs:  {:?}", line.inherent_tags);
    println!("| Description: {}", line.description);
    println!("+------------------+");
}
