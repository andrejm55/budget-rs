# budget-rs

A personal finance tracker built in Rust.

## Features

- Add income and expense transactions
- List saved transactions
- Filter transactions by month
- Filter transactions by category
- Generate summary reports for income, expenses, and net balance
- Delete transactions by ID
- Store data locally in JSON
- Launch a simple local browser GUI
- Includes unit tests for parsing, filtering, and summary calculations

## Tech Stack

### Backend

- Rust
- `clap` for CLI argument parsing
- `serde` and `serde_json` for JSON serialization and storage
- `chrono` for date handling
- `anyhow` for error handling
- `axum` for the local HTTP server and JSON API
- `tokio` as the async runtime for the GUI server

### Frontend

- HTML for the page structure
- CSS for the simple local GUI styling
- Vanilla JavaScript for form submission, filtering, table rendering, and API calls

## Architecture

### Backend

- CLI entry points live in `src/main.rs` and `src/cli.rs`
- Transaction models and enums live in `src/models.rs`
- Reporting and filtering logic live in `src/reports.rs`
- JSON file persistence lives in `src/storage.rs`
- The web server and API endpoints for the GUI live in `src/web.rs`

### Frontend

- `web/index.html` defines the layout
- `web/styles.css` contains the GUI styling
- `web/app.js` handles browser-side interactions and calls the Rust backend API

### How they connect

- The frontend is served by the Rust backend
- Browser actions call local endpoints such as `/api/transactions` and `/api/summary`
- The backend reads and writes transaction data from `data/transactions.json`

## Running the GUI

Start the browser interface with:

```bash
cargo run -- gui
```

Then open [http://127.0.0.1:3000](http://127.0.0.1:3000) in your browser.

You can also choose a custom port:

```bash
cargo run -- gui --port 4000
```
