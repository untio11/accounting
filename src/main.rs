use csv::Reader;
use std::{env, error::Error, fs::File, path, process};
mod parsing;
mod processing;

use crate::{parsing::IngCurrentAccount, processing::types::Transaction};

fn main() {
    if let Err(err) = csv_from_file(get_first_arg()) {
        println!("{}", err);
        process::exit(1);
    }
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> path::PathBuf {
    match env::args_os().nth(1) {
        None => panic!("Required argument: Path pointing to csv file."),
        Some(file_path) => path::PathBuf::from(file_path),
    }
}

fn print_csv_line(line: &Transaction) {
    println!("\n=========================================");
    println!("Amount:      {}", line.amount);
    println!("Date:        {}", line.date);
    println!("Source:      {:?}", line.source);
    println!("Sink:        {:?}", line.sink);
    println!("Inhrnt Tgs:  {:?}", line.inherent_tags);
    println!("Description: {}", line.description);
    println!("=========================================");
}

fn csv_from_file(file_path: path::PathBuf) -> Result<(), Box<dyn Error>> {
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
    for line in parsed_csv {
        print_csv_line(&line);
    }

    Ok(())
}
