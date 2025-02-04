use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::logging_table::query_validator;
use crate::ApiError;
use crate::{app_state::AppState, logging_table::LoggingTable};

#[derive(Deserialize)]
pub struct Request {
    pub query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Option<Vec<LoggingTable>>,
}

pub async fn post_query(
    State(state): State<AppState>,
    Json(input): Json<Request>,
) -> Result<impl IntoResponse, ApiError> {
    let query = match input.query {
        Some(v) => {
            match query_validator(&v) {
                true => v,
                false => return Err(ApiError::IncorrectQuery)
            }
        },
        None => return Err(ApiError::IncorrectQuery)
    };
    let df = state
        .ctx
        .sql(&query)
        .await
        .map_err(|e| ApiError::UnexpectedError(e.into()))?;
    let res = LoggingTable::df_to_records(df)
        .await
        .map_err(|e| ApiError::UnexpectedError(e.into()))?;
    if res.is_empty() {
        return Err(ApiError::QueryResultIsEmpty);
    }

    let response = Response {
        message: "Table selected".to_string(),
        content: Some(res),
    };
    Ok((StatusCode::OK, Json(response)))
}
