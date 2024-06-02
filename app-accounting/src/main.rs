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
use parsing::Direction;
use processing::types::{Account, Node, Transaction};
use rusqlite::Connection;
use rust_decimal::Decimal;

fn insert_node(db: &Connection, node_id: String) {
    db.execute("INSERT OR IGNORE INTO nodes (id) VALUES (?1)", [(node_id)])
        .unwrap();
}

fn insert_transaction(db: &Connection, t: &Transaction) {
    db.execute(
        "INSERT OR REPLACE INTO transactions VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            t.id().to_string(),
            t.date.to_string(),
            t.source.id().to_string(),
            t.sink.id().to_string(),
            (t.amount * Decimal::ONE_HUNDRED).to_string(),
            t.inherent_tags.join(" "),
            t.description.to_owned(),
            match t.direction {
                Direction::Incoming => "Incoming",
                Direction::Outgoing => "Outgoing",
            },
        ),
    )
    .unwrap();
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let db = Connection::open("sandbox/transactions.db")?;
    db.execute(
        "create table if not exists nodes (
            id TEXT PRIMARY KEY
        )",
        (),
    )?;
    db.execute(
        "create table if not exists transactions (
            id TEXT PRIMARY KEY,
            date TEXT NOT NULL,
            sourceid TEXT,
            sinkid TEXT,
            amount TEXT NOT NULL,
            inherenttags TEXT,
            description TEXT NOT NULL,
            direction TEXT NOT NULL,
            FOREIGN KEY(sourceid) REFERENCES nodes(id)
            FOREIGN KEY(sinkid) REFERENCES nodes(id)
        )",
        (),
    )?;
    db.execute("INSERT OR IGNORE INTO nodes (id) VALUES (?1)", [("id-1")])?;
    db.execute("INSERT OR IGNORE INTO nodes (id) VALUES (?1)", [("id-2")])?;
    db.execute(
        "INSERT OR REPLACE INTO transactions VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            "id-0",
            "2024-28-05",
            "id-1",
            "id-2",
            "12",
            "#tag",
            "description placeholder",
            "Incoming",
        ),
    )?;

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
        transactions
            .iter()
            .for_each(|transaction| insert_transaction(&db, transaction));
        let node_freq = summaries::node_frequencies(&transactions);
        let mut node_freq: Vec<_> = node_freq.iter().collect();
        node_freq.sort_unstable_by(|a, b| b.1.cmp(a.1));

        for (id, count) in node_freq.iter() {
            insert_node(&db, id.to_string());
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
