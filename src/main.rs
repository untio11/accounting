use csv::Reader;
use std::{env, error::Error, fs::File, path, process};
mod parsing;
mod processing;

use crate::{parsing::IngCurrentAccount, processing::types::transaction::Transaction};

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

fn print_csv_line(line: &IngCurrentAccount) {
    println!("\n=========================================");
    println!("As IngCurrentAccount:");
    println!("Name:        {}", line.name);
    println!("Amount:      {}", line.amount);
    println!("Date:        {}", line.date);
    println!("Account:     {}", line.account);
    println!(
        "Cntrprty:    {}",
        match &line.counter_party {
            Some(iban) => iban.to_string(),
            None => String::from("-"),
        }
    );
    println!("Direction:   {:?}", line.direction);
    println!("Code:        {:?}", line.code);
    println!("Type:        {}", line.transaction_type);
    println!("Description: {}", line.description);
    println!("Balance:     {}", line.balance);
    println!("Tags:        {}", line.tags);

    println!();

    println!("Amount:      {}", line.amount());
    println!("Date:        {}", line.date());
    println!("Source:      {:?}", line.source());
    println!("Sink:        {:?}", line.sink());
    println!("Inhrnt Tgs:  {:?}", line.inherent_tags());
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

    let mut parsed_csv: Vec<IngCurrentAccount> = Vec::new();

    for entry in reader.deserialize() {
        match entry {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                parsed_csv.push(record);
            }
        }
    }

    println!("Accessing first element:");
    for line in parsed_csv {
        print_csv_line(&line);
    }

    Ok(())
}
