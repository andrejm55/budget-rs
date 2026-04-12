use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "income" => Ok(TransactionType::Income),
            "expense" => Ok(TransactionType::Expense),
            _ => Err(format!("Invalid transaction type: {s}")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Category {
    Salary,
    Rent,
    Groceries,
    Transport,
    Utilities,
    Dining,
    Entertainment,
    Savings,
    Health,
    Other,
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "salary" => Ok(Category::Salary),
            "rent" => Ok(Category::Rent),
            "groceries" => Ok(Category::Groceries),
            "transport" => Ok(Category::Transport),
            "utilities" => Ok(Category::Utilities),
            "dining" => Ok(Category::Dining),
            "entertainment" => Ok(Category::Entertainment),
            "savings" => Ok(Category::Savings),
            "health" => Ok(Category::Health),
            "other" => Ok(Category::Other),
            _ => Err(format!("Invalid category: {s}")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: u32,
    pub date: NaiveDate,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub category: Category,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_transaction_type_income() {
        let parsed = TransactionType::from_str("income").unwrap();
        assert!(matches!(parsed, TransactionType::Income));
    }

    #[test]
    fn parses_transaction_type_expense_case_insensitive() {
        let parsed = TransactionType::from_str("ExPeNsE").unwrap();
        assert!(matches!(parsed, TransactionType::Expense));
    }

    #[test]
    fn rejects_invalid_transaction_type() {
        let parsed = TransactionType::from_str("bonus");
        assert!(parsed.is_err());
    }

    #[test]
    fn parses_category_groceries() {
        let parsed = Category::from_str("groceries").unwrap();
        assert!(matches!(parsed, Category::Groceries));
    }

    #[test]
    fn parses_category_salary_case_insensitive() {
        let parsed = Category::from_str("SaLaRy").unwrap();
        assert!(matches!(parsed, Category::Salary));
    }

    #[test]
    fn rejects_invalid_category() {
        let parsed = Category::from_str("travel");
        assert!(parsed.is_err());
    }
}
