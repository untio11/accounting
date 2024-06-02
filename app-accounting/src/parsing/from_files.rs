use crate::parsing::Direction;
use crate::processing::types::{Transaction, Transactions};
use crate::{parsing::IngCurrentAccount, processing::Identify};
use clap::Parser;
use core::panic;
use std::{
    collections::HashSet,
    error::Error,
    fs::{self, File},
    path,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a .csv file or a directory that contains at
    /// least one .csv file.
    #[arg(short, long)]
    pub path: std::path::PathBuf,
}

/// Deserialize .csv files into a Vector of Transactions.
///
/// `file_path` can point to:
/// - a directory that contains at least 1 .csv file. In this case,
/// all .csv's in that directory will be deserialized.
/// - a single .csv file. In this case, just this .csv file will be
/// deserialized.
///
/// The resulting vector satisfies the following properties:
/// - The transactions are sorted by increasing date, at the granularity
/// of days. Order of transactions occuring on the same day cannot be guaranteed.
/// - The transactions are unique. This is based on the hash of the transaction.
/// Note: this isn't guaranteed to be the same hash ID you get from `Transaction::id()`.
pub fn transactions_from_path(file_path: &path::PathBuf) -> Result<Transactions, Box<dyn Error>> {
    let files = match file_path {
        dirname if file_path.is_dir() => {
            println!("Looking for .csv files in directory: {:?}", dirname);
            let mut files: Vec<path::PathBuf> = vec![];
            for path in fs::read_dir(dirname)? {
                let path = path.unwrap().path();
                if path.is_file() && path.extension().unwrap() == "csv" {
                    files.push(path);
                }
            }
            if files.is_empty() {
                panic!("The directory: {:?} contains no .csv files.", dirname);
            }
            files
        }
        csv_file if file_path.is_file() && file_path.extension().unwrap() == "csv" => {
            vec![path::PathBuf::clone(csv_file)]
        }
        _ => panic!("Expecting a path to a directory or a .csv file"),
    };

    let mut transactions: Vec<Transaction> = Vec::new();
    println!("Reading:");
    for file in files {
        if let Ok(file) = File::open(file) {
            println!("> {:?}", file);
            transactions.append(&mut read_transactions_from(file));
        }
    }

    println!("Deduplicating transactions");
    let before = transactions.len();
    deduplicate(&mut transactions);
    println!(
        "> Removed {:?} duplicate transaction(s)",
        before - transactions.len()
    );

    println!("Sorting transactions on date");
    transactions.sort_by(|a, b| a.date.cmp(&b.date));

    println!("Accessing first 5 elements:");
    for line in &transactions[..5] {
        print_csv_line(line);
    }

    Ok(transactions)
}

/// Deserialize the transactions in a single .csv file. At this point, there
/// are no guarantees about uniqueness or order.
///
/// Currently only supports hardcoded deserialization from `IngCurrentAccount`.
fn read_transactions_from(file: File) -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';') // Perhaps csv file specific.
        .flexible(true)
        .from_reader(file);
    for row in reader.deserialize::<IngCurrentAccount>().flatten() {
        transactions.push(Transaction::from(row));
    }
    transactions
}

/// Remove duplicate transactions from the vector. This leaves the transactions
/// in random order.
fn deduplicate(transactions: &mut Vec<Transaction>) -> &mut Vec<Transaction> {
    // itertools dedup.
    let set: HashSet<_> = transactions.drain(..).collect(); // dedup
    transactions.extend(set);
    transactions
}

pub fn print_csv_line(line: &Transaction) {
    println!("\n+==================+");
    println!(
        "| {} | ({})",
        line.id(),
        if line.direction == Direction::Incoming {
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
