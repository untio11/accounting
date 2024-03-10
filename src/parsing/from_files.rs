use crate::parsing::IngCurrentAccount;
use crate::processing::types::Transaction;
use clap::Parser;
use core::panic;
use csv::Reader;
use std::{error::Error, fs::File, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a .csv file or a directory that contains at
    /// least one .csv file.
    #[arg(short, long)]
    pub path: std::path::PathBuf,
}

pub fn csv_from_file(file_path: &path::PathBuf) -> Result<(), Box<dyn Error>> {
    let files = match file_path {
        dirname if file_path.is_dir() => {
            println!("dirname: {:?}", dirname);
        }
        csv_file if file_path.is_file() && file_path.extension().unwrap() == "csv" => {
            println!("csv_file: {:?}", csv_file);
        }
        _ => panic!("Expecting a path to a directory or a .csv file"),
    };
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(From::from(err)),
    };

    let mut reader: Reader<File> = csv::ReaderBuilder::new()
        .delimiter(b';') // Perhaps csv file specific.
        .flexible(true)
        .from_reader(file);

    {
        let headers = reader.headers().expect("There should be headers here fr.");
        println!("Headers: \n{:?}", headers);
    }

    let mut parsed_csv: Vec<Transaction> = Vec::new();

    for entry in reader.deserialize::<IngCurrentAccount>() {
        match entry {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                parsed_csv.push(Transaction::from(record));
            }
        }
    }

    println!("Accessing first element:");
    for line in &parsed_csv[..=5] {
        print_csv_line(&line);
    }

    Ok(())
}

pub fn print_csv_line(line: &Transaction) {
    println!("\n=========================================");
    println!("Amount:      {}", line.amount);
    println!("Date:        {}", line.date);
    println!("Source:      {:?}", line.source);
    println!("Sink:        {:?}", line.sink);
    println!("Inhrnt Tgs:  {:?}", line.inherent_tags);
    println!("Description: {}", line.description);
    println!("=========================================");
}
