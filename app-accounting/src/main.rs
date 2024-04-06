mod parsing;
mod processing;
mod state;
use crate::{
    parsing::transactions_from_path,
    processing::{summaries, types::SubAccount, Identify},
    state::Owner,
};
use clap::Parser;
use color_eyre::Result;
use iban::Iban;
use processing::types::{Account, Node};
// use rusqlite::Connection;

fn main() -> Result<()> {
    color_eyre::install()?;
    // let db = Connection::open("sandbox/database.db")?;
    // db.execute(
    //     "create table if not exists transactions (
    //          id integer primary key,
    //          name text not null unique
    //      )",
    //     NO_PARAMS,
    // )?;
    let args = crate::parsing::Args::parse();
    println!("{:?}", args);
    let my_account = Account {
        iban: Iban::parse("NL95INGB0756630126").unwrap(),
        name: String::from("My Account"),
    };
    let me = Owner {
        name: "Robin",
        owned: vec![
            Node::ProperAccount(my_account.clone()),
            Node::ProperAccount(Account {
                iban: Iban::parse("NL18BUNQ2091852465").unwrap(),
                name: String::from("My Bunq Account"),
            }),
            Node::SubAccount(SubAccount {
                bsan: String::from("S96846640"),
                name: String::from("Leuke Dingen"),
                parent_account: Some(my_account.id()),
                account_type: Some(processing::types::AccountType::Saving),
            }),
            Node::SubAccount(SubAccount {
                bsan: String::from("14742098"),
                name: String::from("Beleggingsrekening"),
                parent_account: Some(my_account.id()),
                account_type: Some(processing::types::AccountType::Brokerage),
            }),
            Node::SubAccount(SubAccount {
                bsan: String::from("T78658603"),
                name: String::from("Buffer"),
                parent_account: Some(my_account.id()),
                account_type: Some(processing::types::AccountType::Saving),
            }),
        ],
    };

    if let Ok(transactions) = transactions_from_path(&args.path) {
        let node_freq = summaries::node_frequencies(transactions);
        let mut node_freq: Vec<_> = node_freq.iter().collect();
        node_freq.sort_unstable_by(|a, b| b.1.cmp(a.1));

        for (id, count) in node_freq.iter() {
            println!(
                "{:?}: {count}",
                if let Some(node) = me.owned.iter().find(|node| node.id() == *(*id)) {
                    node.to_string()
                } else {
                    id.to_string()
                }
            );
        }

        return Ok(());
    }
    panic!("Couldn't parse transactions successfully.");
}
