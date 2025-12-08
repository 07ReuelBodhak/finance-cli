use crate::models::transaction::{Transaction,ExpenseType};
use chrono::prelude::*;
use crate::parser::parser::Parse;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub struct Ledger{
    processed_transactions : Vec<Transaction>,
}

fn load_transactions_from_file() -> Vec<Transaction> {
    let path = "ledger.jsonl";
    let mut transactions = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(tx) = serde_json::from_str::<Transaction>(&line) {
                    transactions.push(tx);
                }
            }
        }
    }
    transactions
}


#[allow(dead_code)]
impl Ledger {
    pub fn new()-> Self {
        Self {
            processed_transactions : load_transactions_from_file(),
        }
    }

    pub fn add_transaction(&mut self, description:String, category:String, amount:f64, expense_type:ExpenseType){
        let date = Local::now().format("%Y-%m-%d").to_string();

        let transaction = Transaction {
            id: self.processed_transactions.len() + 1,
            amount,
            description,
            date,
            category,
            expense_type,
        };
        self.processed_transactions.push(transaction.clone());

        match Self::parse_transaction(&transaction) {
            Ok(_) => println!("Transaction added successfully!"),
            Err(e) => eprintln!("Failed to save ledger: {}", e),
        }
    }
}

impl Parse for Ledger {}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works(){
        let ledger = Ledger::new();
        assert_eq!(ledger.processed_transactions.len(),0);
    }

    #[test]
    fn add_expense(){
        let mut ledger = Ledger::new();
        let transaction = Transaction {
            id : 1,
            amount : 100.0,
            description : "Test Transaction".to_string(),
            date : "2023-10-10".to_string(),
            category : "Test".to_string(),
            expense_type:ExpenseType::Spend
        };
        ledger.add_transaction(transaction.description, transaction.category, transaction.amount, transaction.expense_type);
        assert_eq!(ledger.processed_transactions.len(),1);
    }
}