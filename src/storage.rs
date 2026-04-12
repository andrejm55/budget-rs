use crate::models::Transaction;
use anyhow::Result;
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "data/transactions.json";

pub fn load_transactions() -> Result<Vec<Transaction>> {
    if !Path::new(DATA_FILE).exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(DATA_FILE)?;
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let transactions: Vec<Transaction> = serde_json::from_str(&contents)?;
    Ok(transactions)
}

pub fn save_transactions(transactions: &[Transaction]) -> Result<()> {
    if let Some(parent) = Path::new(DATA_FILE).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(transactions)?;
    fs::write(DATA_FILE, json)?;
    Ok(())
}
