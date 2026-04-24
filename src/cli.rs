use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "budget-rs")]
#[command(version = "0.1.0")]
#[command(about = "A simple personal finance tracker")]
#[command(
    long_about = "budget-rs is a command-line personal finance tracker for recording income and expenses, filtering transactions, and generating summaries."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Add a new income or expense transaction")]
    Add {
        #[arg(help = "Transaction type: income or expense")]
        transaction_type: String,

        #[arg(help = "Transaction amount, for example 24.50")]
        amount: f64,

        #[arg(help = "Category, for example groceries, salary, or transport")]
        category: String,

        #[arg(help = "Short description of the transaction")]
        description: String,

        #[arg(long, help = "Transaction date in YYYY-MM-DD format")]
        date: Option<String>,
    },

    #[command(about = "List saved transactions")]
    List {
        #[arg(long, help = "Filter by month in YYYY-MM format")]
        month: Option<String>,

        #[arg(long, help = "Filter by category")]
        category: Option<String>,
    },

    #[command(about = "Show income, expense, and net balance summary")]
    Summary {
        #[arg(long, help = "Filter summary by month in YYYY-MM format")]
        month: Option<String>,
    },

    #[command(about = "Delete a transaction by ID")]
    Delete {
        #[arg(help = "Transaction ID to delete")]
        id: u32,
    },

    #[command(about = "Launch the local browser-based GUI")]
    Gui {
        #[arg(long, default_value_t = 3000, help = "Port to serve the GUI on")]
        port: u16,
    },
}
