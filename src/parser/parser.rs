use crate::models::transaction::Transaction;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json;

pub trait Parse {
    fn parse_transaction(transaction: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("ledger.jsonl")?;

        let json = serde_json::to_string(transaction)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }
}