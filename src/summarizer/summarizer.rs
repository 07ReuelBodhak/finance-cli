use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::transaction::{Transaction, ExpenseType};

pub fn summary() {
    let file = match File::open("ledger.jsonl") {
        Ok(f) => f,
        Err(_) => {
            println!("No transactions found.");
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();

    // Read JSONL and parse into struct
    for line in reader.lines() {
        if let Ok(json_line) = line {
            if !json_line.trim().is_empty() {
                if let Ok(tx) = serde_json::from_str::<Transaction>(&json_line) {
                    transactions.push(tx);
                }
            }
        }
    }

    if transactions.is_empty() {
        println!("No transactions available.");
        return;
    }

    // ---------- SUMMARIES ----------
    let mut total_income = 0.0;
    let mut total_expense = 0.0;

    use std::collections::HashMap;
    let mut category_expenses: HashMap<String, f64> = HashMap::new();

    for tx in &transactions {
        match tx.expense_type {
            ExpenseType::Earned => total_income += tx.amount,
            ExpenseType::Spend  => {
                total_expense += tx.amount;

                *category_expenses
                    .entry(tx.category.clone())
                    .or_insert(0.0) += tx.amount;
            }
        }
    }

    let net_balance = total_income - total_expense;
    let balance_status = if net_balance >= 0.0 { "Positive" } else { "Negative" };

    // ---------- PRINT SUMMARY ----------
    println!("================= FINANCE SUMMARY =================\n");

    println!("Total Income      : ₹ {:>10.2}", total_income);
    println!("Total Expenses    : ₹ {:>10.2}", total_expense);
    println!("---------------------------------------------------");
    println!("Net Balance       : ₹ {:>10.2}   ({})\n", net_balance, balance_status);

    println!("------------ Category Breakdown (Expenses) --------");

    let mut categories: Vec<_> = category_expenses.iter().collect();
    categories.sort_by_key(|&(cat, _)| cat.to_string());

    for (cat, amt) in categories {
        println!("{:<16}: ₹ {:>8.2}", cat, amt);
    }

    println!("\n-------------- Recent Transactions ----------------");

    for tx in &transactions {
        println!(
            "[{}] {} | {:<6} | {:<15} : ₹ {:>6.0}",
            tx.id,
            tx.date,
            match tx.expense_type { ExpenseType::Earned => "Earned", ExpenseType::Spend => "Spend" },
            tx.description,
            tx.amount
        );
    }

    println!("\n===================================================");
}
