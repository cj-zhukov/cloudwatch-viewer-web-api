use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use color_eyre::eyre::Report;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::utils::tracing::log_error_chain;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Incorrect query")]
    IncorrectQuery,

    #[error("Query result is empty")]
    QueryResultIsEmpty,

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        log_error_chain(&self);

        let (status, error_message) = match self {
            ApiError::IncorrectQuery => (StatusCode::BAD_REQUEST, "Incorrect query"),
            ApiError::QueryResultIsEmpty => (StatusCode::NOT_FOUND, "Not found"),
            ApiError::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
