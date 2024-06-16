mod parsing;
mod processing;
mod state;
use std::path::PathBuf;

use crate::{
    parsing::transactions_from_path,
    processing::{summaries, Identify},
};
use clap::Parser;
use color_eyre::Result;
use iban::Iban;
use itertools::Itertools;
use parsing::{profile_from_path, Direction};
use processing::types::{Account, Node, SubAccount, Transaction};
use rusqlite::Connection;
use rust_decimal::Decimal;
use state::Owner;

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
    let me = profile_from_path(&args.profile_path);
    if let Ok(transactions) = transactions_from_path(&args.csv_path) {
        let node_freq = summaries::node_frequencies(&transactions);
        let node_freq: Vec<_> = node_freq.iter().sorted_by(|a, b| b.1.cmp(a.1)).collect();

        for (id, count) in node_freq.iter() {
            insert_node(&db, id.to_string());
            println!("{:?}: {count}", id.to_string());
        }

        println!(
            "Date range: {:?}",
            [transactions.first(), transactions.last()].map(|t| t.unwrap().date)
        );

        return Ok(());
    }
    panic!("Couldn't parse transactions successfully.");
}
