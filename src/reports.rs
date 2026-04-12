use crate::models::{Category, Transaction, TransactionType};
use std::str::FromStr;

pub fn filter_transactions(
    transactions: &[Transaction],
    month: Option<&str>,
    category: Option<&str>,
) -> Result<Vec<Transaction>, String> {
    let mut filtered: Vec<Transaction> = transactions.to_vec();

    if let Some(month_str) = month {
        filtered.retain(|tx| tx.date.format("%Y-%m").to_string() == month_str);
    }

    if let Some(category_str) = category {
        let parsed_category = Category::from_str(category_str)?;
        filtered.retain(|tx| {
            std::mem::discriminant(&tx.category) == std::mem::discriminant(&parsed_category)
        });
    }

    Ok(filtered)
}

pub fn calculate_summary(transactions: &[Transaction]) -> (f64, f64, f64) {
    let mut total_income = 0.0;
    let mut total_expenses = 0.0;

    for tx in transactions {
        match tx.transaction_type {
            TransactionType::Income => total_income += tx.amount,
            TransactionType::Expense => total_expenses += tx.amount,
        }
    }

    let net_balance = total_income - total_expenses;
    (total_income, total_expenses, net_balance)
}

pub fn format_transaction_type(transaction_type: &TransactionType) -> &'static str {
    match transaction_type {
        TransactionType::Income => "Income",
        TransactionType::Expense => "Expense",
    }
}

pub fn format_category(category: &Category) -> &'static str {
    match category {
        Category::Salary => "Salary",
        Category::Rent => "Rent",
        Category::Groceries => "Groceries",
        Category::Transport => "Transport",
        Category::Utilities => "Utilities",
        Category::Dining => "Dining",
        Category::Entertainment => "Entertainment",
        Category::Savings => "Savings",
        Category::Health => "Health",
        Category::Other => "Other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn sample_transactions() -> Vec<Transaction> {
        vec![
            Transaction {
                id: 1,
                date: NaiveDate::from_ymd_opt(2026, 4, 1).unwrap(),
                transaction_type: TransactionType::Income,
                amount: 1800.0,
                category: Category::Salary,
                description: "Monthly salary".to_string(),
            },
            Transaction {
                id: 2,
                date: NaiveDate::from_ymd_opt(2026, 4, 12).unwrap(),
                transaction_type: TransactionType::Expense,
                amount: 24.5,
                category: Category::Groceries,
                description: "Tesco shop".to_string(),
            },
            Transaction {
                id: 3,
                date: NaiveDate::from_ymd_opt(2026, 4, 13).unwrap(),
                transaction_type: TransactionType::Expense,
                amount: 12.5,
                category: Category::Transport,
                description: "Bus fare".to_string(),
            },
            Transaction {
                id: 4,
                date: NaiveDate::from_ymd_opt(2026, 3, 28).unwrap(),
                transaction_type: TransactionType::Expense,
                amount: 40.0,
                category: Category::Dining,
                description: "Dinner out".to_string(),
            },
        ]
    }

    #[test]
    fn filters_by_month() {
        let transactions = sample_transactions();
        let filtered = filter_transactions(&transactions, Some("2026-04"), None).unwrap();

        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn filters_by_category() {
        let transactions = sample_transactions();
        let filtered = filter_transactions(&transactions, None, Some("groceries")).unwrap();

        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0].category, Category::Groceries));
    }

    #[test]
    fn filters_by_month_and_category() {
        let transactions = sample_transactions();
        let filtered =
            filter_transactions(&transactions, Some("2026-04"), Some("transport")).unwrap();

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].description, "Bus fare");
    }

    #[test]
    fn rejects_invalid_category_filter() {
        let transactions = sample_transactions();
        let filtered = filter_transactions(&transactions, None, Some("travel"));

        assert!(filtered.is_err());
    }

    #[test]
    fn calculates_summary_correctly() {
        let transactions = sample_transactions();
        let filtered = filter_transactions(&transactions, Some("2026-04"), None).unwrap();
        let (income, expenses, net) = calculate_summary(&filtered);

        assert_eq!(income, 1800.0);
        assert_eq!(expenses, 37.0);
        assert_eq!(net, 1763.0);
    }
}
