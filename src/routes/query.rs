use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{app_state::AppState, logging_table::LoggingTable};
use crate::LOGGING_TABLE_NAME;

pub async fn get_query(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let query = format!("select * from {LOGGING_TABLE_NAME} limit 10");
    let df = state.ctx.sql(&query).await.unwrap();
    let records = LoggingTable::df_to_records(df).await.unwrap();    
    let res = serde_json::to_string(&records).unwrap();
    (StatusCode::OK, res)
}