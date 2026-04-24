mod cli;
mod models;
mod reports;
mod storage;
mod web;

use anyhow::{Result, anyhow};
use chrono::NaiveDate;
use clap::Parser;
use cli::{Cli, Commands};
use models::{Category, Transaction, TransactionType};
use reports::{calculate_summary, filter_transactions, format_category, format_transaction_type};
use std::str::FromStr;
use storage::{load_transactions, save_transactions};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { month, category } => {
            let transactions = load_transactions()?;
            let filtered =
                filter_transactions(&transactions, month.as_deref(), category.as_deref())
                    .map_err(|e| anyhow!(e))?;

            if filtered.is_empty() {
                println!("No transactions found for the selected filters.");
                return Ok(());
            }

            println!("Loaded {} matching transaction(s)", filtered.len());
            println!();
            println!(
                "{:<4} {:<12} {:<10} {:<10} {:<15} {}",
                "ID", "Date", "Type", "Amount", "Category", "Description"
            );

            for tx in filtered {
                println!(
                    "{:<4} {:<12} {:<10} {:<10.2} {:<15} {}",
                    tx.id,
                    tx.date,
                    format_transaction_type(&tx.transaction_type),
                    tx.amount,
                    format_category(&tx.category),
                    tx.description
                );
            }
        }
        Commands::Add {
            transaction_type,
            amount,
            category,
            description,
            date,
        } => {
            if amount <= 0.0 {
                return Err(anyhow!("Amount must be greater than 0."));
            }

            let mut transactions = load_transactions()?;

            let parsed_type =
                TransactionType::from_str(&transaction_type).map_err(|e| anyhow!(e))?;

            let parsed_category = Category::from_str(&category).map_err(|e| anyhow!(e))?;

            let parsed_date = match date {
                Some(d) => NaiveDate::parse_from_str(&d, "%Y-%m-%d")?,
                None => chrono::Local::now().date_naive(),
            };

            let next_id = transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            let transaction = Transaction {
                id: next_id,
                date: parsed_date,
                transaction_type: parsed_type,
                amount,
                category: parsed_category,
                description,
            };

            transactions.push(transaction);
            save_transactions(&transactions)?;

            println!("Transaction added successfully.");
        }
        Commands::Summary { month } => {
            let transactions = load_transactions()?;
            let filtered = filter_transactions(&transactions, month.as_deref(), None)
                .map_err(|e| anyhow!(e))?;

            match month {
                Some(m) => println!("Summary for {m}"),
                None => println!("Summary for all transactions"),
            }

            if filtered.is_empty() {
                println!();
                println!("No transactions found for this summary.");
                return Ok(());
            }

            let (income, expenses, net) = calculate_summary(&filtered);

            println!();
            println!("Total income:   {:.2}", income);
            println!("Total expenses: {:.2}", expenses);
            println!("Net balance:    {:.2}", net);
        }
        Commands::Delete { id } => {
            let mut transactions = load_transactions()?;
            let original_len = transactions.len();

            transactions.retain(|tx| tx.id != id);

            if transactions.len() == original_len {
                println!("No transaction found with ID {}.", id);
                return Ok(());
            }

            save_transactions(&transactions)?;
            println!("Transaction {} deleted successfully.", id);
        }
        Commands::Gui { port } => {
            web::run_server(port).await?;
        }
    }

    Ok(())
}
