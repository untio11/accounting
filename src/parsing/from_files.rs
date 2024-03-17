use crate::processing::types::Transaction;
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

pub fn csv_from_path(file_path: &path::PathBuf) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let files = match file_path {
        dirname if file_path.is_dir() => {
            println!("dirname: {:?}", dirname);
            let mut files: Vec<path::PathBuf> = vec![];
            for path in fs::read_dir(dirname)? {
                let path = path.unwrap().path();
                if path.is_file() && path.extension().unwrap() == "csv" {
                    files.push(path);
                }
            }
            if files.len() == 0 {
                panic!("The directory: {:?} contains no .csv files.", dirname);
            }
            files
        }
        csv_file if file_path.is_file() && file_path.extension().unwrap() == "csv" => {
            println!("csv_file: {:?}", csv_file);
            vec![path::PathBuf::clone(csv_file)]
        }
        _ => panic!("Expecting a path to a directory or a .csv file"),
    };

    let mut transactions: Vec<Transaction> = Vec::new();
    for file in files {
        if let Ok(file) = File::open(file) {
            println!("Reading: {:?}", file);
            transactions.append(&mut read_transactions_from(file));
        }
    }

    println!("Deduplicating transactions");
    let before = transactions.len();
    deduplicate(&mut transactions);
    println!(
        "Removed {:?} duplicate transaction(s)",
        before - transactions.len()
    );

    println!("Accessing first element:");
    for line in &transactions[..] {
        print_csv_line(&line);
    }

    Ok(transactions)
}

fn read_transactions_from(file: File) -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';') // Perhaps csv file specific.
        .flexible(true)
        .from_reader(file);
    for row in reader.deserialize::<IngCurrentAccount>() {
        match row {
            Ok(row) => {
                transactions.push(Transaction::from(row));
            }
            _ => (),
        }
    }
    return transactions;
}

fn deduplicate(transactions: &mut Vec<Transaction>) -> &mut Vec<Transaction> {
    let set: HashSet<_> = transactions.drain(..).collect(); // dedup
    transactions.extend(set.into_iter());
    transactions.sort_by(|a, b| a.date.cmp(&b.date));
    return transactions;
}

pub fn print_csv_line(line: &Transaction) {
    println!("\n+==================+");
    println!("| {:X} |", line.id());
    println!("+==================+");
    println!("| Amount:      {}", line.amount);
    println!("| Date:        {}", line.date);
    println!("| Source:      {}", line.source);
    println!("| Sink:        {}", line.sink);
    println!("| Inhrnt Tgs:  {:?}", line.inherent_tags);
    println!("| Description: {}", line.description);
    println!("+------------------+");
}
