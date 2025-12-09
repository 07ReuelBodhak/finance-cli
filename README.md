# Finance CLI

A simple, fast, and lightweight **personal finance tracker** built in Rust.
It allows you to record income and expenses, automatically append them to a `.jsonl` ledger file, and generate clean financial summaries directly from your terminal.

---

## Features

- Add **income** and **expense** transactions
- Data stored in `ledger.jsonl` using JSON Lines format
- Summaries include:

  - Total income
  - Total expenses
  - Net balance (positive / negative)
  - Category-wise expense breakdown
  - Recent transactions

- Fast CLI performance with safe Rust file handling
- Auto-incrementing transaction IDs

---

## Installation

Clone the repository:

```bash
git clone https://github.com/07ReuelBodhak/finance-cli.git
cd finance-cli
```

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run -- <command>
```

---

## Usage

### 1. Add an Income

```bash
cargo run -- income "<description>" <category> <amount>
```

Example:

```bash
cargo run -- income "Salary" Job 30000
```

### 2. Add an Expense

```bash
cargo run -- expense "<description>" <category> <amount>
```

Example:

```bash
cargo run -- expense "Pani Puri" Food 100
```

### 3. View Summary

```bash
cargo run -- summary
```

Generates a formatted financial overview.

---

## Data Format

Transactions are stored in `ledger.jsonl` as JSON Lines:

```
{"id":1,"amount":30000.0,"description":"1 month Salary","date":"2025-12-08","category":"Salary","expense_type":"Earned"}
{"id":2,"amount":100.0,"description":"Pani Puri","date":"2025-12-08","category":"Food","expense_type":"Spend"}
```

This format makes it easy to append, read, or pipe into other tools.

---

## Example Summary Output

```
================= FINANCE SUMMARY =================

Total Income      : ₹ 30,000.00
Total Expenses    : ₹    100.00
---------------------------------------------------
Net Balance       : ₹ 29,900.00   (Positive)

------------ Category Breakdown (Expenses) --------
Food              : ₹ 100.00
Transport         : ₹ 0.00
Shopping          : ₹ 0.00
Entertainment     : ₹ 0.00
Other             : ₹ 0.00

-------------- Recent Transactions ----------------
[1] 2025-12-08 | Earned | 1 month Salary : ₹ 30,000
[2] 2025-12-08 | Spend  | Pani Puri      : ₹   100

===================================================
```

---

## File Structure

```
src/
│
├── collections/
│   ├── ledger.rs          # File read/write, append, auto-ID generation
│   └── mod.rs
│
├── config/
│   ├── config.rs          # Future: global settings, path config, currency
│   └── mod.rs
│
├── models/
│   ├── transaction.rs     # Struct Transaction, serde derive
│   └── mod.rs
│
├── parser/
│   ├── parser.rs          # CLI parsing logic: expense, income, summary
│   └── mod.rs
│
├── summarizer/
│   ├── summarizer.rs      # Summary generation, category totals, prints
│   └── mod.rs
│
├── main.rs                # Entry point – wires parser + actions together
│
├── ledger.jsonl           # JSON Lines ledger file
└── .gitignore

```
