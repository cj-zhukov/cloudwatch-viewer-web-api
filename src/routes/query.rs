use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

// use crate::error::ClouWatchViewerError;
use crate::{app_state::AppState, logging_table::LoggingTable};
use crate::LOGGING_TABLE_NAME;

#[derive(Deserialize)]
pub struct Request {
    pub query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Vec<LoggingTable>,
}

pub async fn post_query(
    State(state): State<AppState>,
    Json(input): Json<Request>
) -> impl IntoResponse {
    let query = match input.query {
        Some(v) => v,
        None => format!("select * from {LOGGING_TABLE_NAME} limit 10")
    };
    let df = state.ctx.sql(&query).await.unwrap();
    let res = LoggingTable::df_to_records(df).await.unwrap();
    let response = Response {
        message: "Table selected".to_string(),
        content: res,
    };
    (StatusCode::OK, Json(response))
}
