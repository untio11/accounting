use csv::Reader;
use std::{env, error::Error, fs::File, path, process};

mod parsing;
use crate::parsing::IngCurrentAccount;

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
        None => panic!("Need 1 argument to csv file to read."),
        Some(file_path) => path::PathBuf::from(file_path),
    }
}

fn csv_from_file(file_path: path::PathBuf) -> Result<(), Box<dyn Error>> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(From::from(err)),
    };

    let mut reader: Reader<File> = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(file);

    {
        let headers = reader.headers().expect("There should be headers here fr.");
        println!("Headers: \n{:?}", headers);
    }

    for entry in reader.deserialize() {
        match entry {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                let record: IngCurrentAccount = record;
                // println!("{:#?}\n", record);
                println!("Parsed Row:\n");
                println!("Name:        {}", record.name);
                println!("Amount:      {}", record.amount);
                println!("Date:        {}", record.date);
                println!("Account:     {}", record.account);
                println!(
                    "Cntrprty:    {}",
                    match record.counter_party {
                        Some(iban) => iban.to_string(),
                        None => String::from("-"),
                    }
                );
                println!("Direction:   {:?}", record.direction);
                println!("Code:        {:?}", record.code);
                println!("Type:        {}", record.transaction_type);
                println!("Description: {}", record.description);
                println!("Balance:     {}", record.balance);
                println!("Tags:        {}", record.tags);

                println!();
            }
        }
    }

    Ok(())
}
