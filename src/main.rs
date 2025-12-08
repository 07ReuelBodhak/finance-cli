mod models;
mod collections;
mod config;
mod parser;

use std::env;
use std::process;
use config::config::Config;
use collections::ledger::Ledger;
use models::transaction::{ExpenseType};

fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::build(args.into_iter())
    .unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments : {err}");
        process::exit(1);
    });

    match config.command {
        config::config::Command::Expense { description, category, amount } => {
            let mut ledger = Ledger::new();
            ledger.add_transaction(description, category, amount.parse().unwrap(),ExpenseType::Spend);
        }
        config::config::Command::Summary => {
            println!("Showing summary:");
        }
        config::config::Command::Income { description, category, amount } => {
            let mut ledger = Ledger::new();
            ledger.add_transaction(description, category, amount.parse().unwrap(),ExpenseType::Earned);
        }
    }
}
