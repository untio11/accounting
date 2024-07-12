use clap::Parser;
use color_eyre::Result;
use itertools::Itertools;

mod analysis;
mod canonical;
mod from_files;
use crate::{
    analysis::summaries,
    canonical::{identify::*, transaction::*},
    from_files::import::{profile_from_path, transactions_from_path},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a .csv file or a directory that contains at least one .csv file.
    #[arg(short, long)]
    pub csv_path: std::path::PathBuf,
    /// Path to a profile .json file.
    #[arg(short, long)]
    pub profile_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
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
