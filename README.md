# budget-rs

A personal finance tracker built in Rust.

Features

- Add income and expense transactions
- List saved transactions
- Filter transactions by month
- Filter transactions by category
- Generate summary reports for income, expenses, and net balance
- Delete transactions by ID
- Store data locally in JSON
- Includes unit tests for parsing, filtering, and summary calculations

Tech Stack

- Rust
- clap for CLI argument parsing
- serde and serde_json for JSON serialization
- chrono for date handling
- anyhow for error handling
