use crate::models::{Category, Transaction, TransactionType};
use crate::reports::{
    calculate_summary, filter_transactions, format_category, format_transaction_type,
};
use crate::storage::{load_transactions, save_transactions};
use anyhow::Result;
use axum::extract::{Path, Query};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{delete, get};
use axum::{Json, Router};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;

const INDEX_HTML: &str = include_str!("../web/index.html");
const APP_JS: &str = include_str!("../web/app.js");
const STYLES_CSS: &str = include_str!("../web/styles.css");

#[derive(Serialize)]
struct TransactionDto {
    id: u32,
    date: String,
    transaction_type: String,
    amount: f64,
    category: String,
    description: String,
}

#[derive(Serialize)]
struct SummaryDto {
    income: f64,
    expenses: f64,
    net: f64,
    transaction_count: usize,
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Deserialize)]
struct CreateTransactionRequest {
    transaction_type: String,
    amount: f64,
    category: String,
    description: String,
    date: Option<String>,
}

pub async fn run_server(port: u16) -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/app.js", get(app_js))
        .route("/styles.css", get(styles_css))
        .route(
            "/api/transactions",
            get(list_transactions).post(create_transaction),
        )
        .route("/api/transactions/{id}", delete(delete_transaction))
        .route("/api/summary", get(get_summary));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Budget GUI available at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn app_js() -> Response {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/javascript"),
        )],
        APP_JS,
    )
        .into_response()
}

async fn styles_css() -> Response {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )],
        STYLES_CSS,
    )
        .into_response()
}

async fn list_transactions(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<TransactionDto>>, AppError> {
    let transactions = load_transactions().map_err(AppError::from)?;
    let filtered = filter_transactions(
        &transactions,
        params.get("month").map(String::as_str),
        params.get("category").map(String::as_str),
    )
    .map_err(AppError::bad_request)?;

    Ok(Json(filtered.into_iter().map(to_transaction_dto).collect()))
}

async fn get_summary(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<SummaryDto>, AppError> {
    let transactions = load_transactions().map_err(AppError::from)?;
    let filtered =
        filter_transactions(&transactions, params.get("month").map(String::as_str), None)
            .map_err(AppError::bad_request)?;
    let (income, expenses, net) = calculate_summary(&filtered);

    Ok(Json(SummaryDto {
        income,
        expenses,
        net,
        transaction_count: filtered.len(),
    }))
}

async fn create_transaction(
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionDto>, AppError> {
    if payload.amount <= 0.0 {
        return Err(AppError::bad_request("Amount must be greater than 0."));
    }

    let parsed_type =
        TransactionType::from_str(&payload.transaction_type).map_err(AppError::bad_request)?;
    let parsed_category = Category::from_str(&payload.category).map_err(AppError::bad_request)?;
    let parsed_date = match payload.date {
        Some(date) if !date.trim().is_empty() => NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(|_| AppError::bad_request("Date must use YYYY-MM-DD format."))?,
        _ => Local::now().date_naive(),
    };

    let description = payload.description.trim();
    if description.is_empty() {
        return Err(AppError::bad_request("Description cannot be empty."));
    }

    let mut transactions = load_transactions().map_err(AppError::from)?;
    let next_id = transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let transaction = Transaction {
        id: next_id,
        date: parsed_date,
        transaction_type: parsed_type,
        amount: payload.amount,
        category: parsed_category,
        description: description.to_string(),
    };

    transactions.push(transaction.clone());
    save_transactions(&transactions).map_err(AppError::from)?;

    Ok(Json(to_transaction_dto(transaction)))
}

async fn delete_transaction(Path(id): Path<u32>) -> Result<Json<MessageResponse>, AppError> {
    let mut transactions = load_transactions().map_err(AppError::from)?;
    let original_len = transactions.len();
    transactions.retain(|tx| tx.id != id);

    if transactions.len() == original_len {
        return Err(AppError::not_found(format!(
            "No transaction found with ID {id}."
        )));
    }

    save_transactions(&transactions).map_err(AppError::from)?;

    Ok(Json(MessageResponse {
        message: format!("Transaction {id} deleted successfully."),
    }))
}

fn to_transaction_dto(tx: Transaction) -> TransactionDto {
    TransactionDto {
        id: tx.id,
        date: tx.date.to_string(),
        transaction_type: format_transaction_type(&tx.transaction_type).to_string(),
        amount: tx.amount,
        category: format_category(&tx.category).to_string(),
        description: tx.description,
    }
}

struct AppError {
    status: StatusCode,
    message: String,
}

impl AppError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(MessageResponse {
            message: self.message,
        });
        (self.status, body).into_response()
    }
}
