use std::error::Error;
use std::time::Duration;
use std::sync::Arc;
use std::path::Path;
use std::fs::File;

use axum::{body::Body, extract::Request, response::Response};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{Level, Span};
use uuid::Uuid;

use crate::error::ApiError;

pub fn init_tracing() -> Result<(), ApiError> {
    let fmt_layer = fmt::layer().compact();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map_err(|e| ApiError::UnexpectedError(e.into()))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();
    Ok(())
}

pub fn init_tracing_using_file(file_path: &str) -> Result<(), ApiError> {
    let log_file = match Path::new(file_path).exists() {
        true => File::open(file_path).map_err(|e| ApiError::UnexpectedError(e.into()))?,
        false => File::create(file_path).map_err(|e| ApiError::UnexpectedError(e.into()))?
    };
    let file_writer = Arc::new(log_file);
    let make_writer = BoxMakeWriter::new(file_writer);

    let file_layer = fmt::Layer::default()
        .with_writer(make_writer) // Send logs to the file
        .with_target(false); // Optional: disable logging module paths

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map_err(|e| ApiError::UnexpectedError(e.into()))?;

    tracing_subscriber::registry()
        .with(filter_layer) // Add the filter layer to control log verbosity
        .with(file_layer) // Add the formatting layer for compact log output
        .with(ErrorLayer::default()) // Add the error layer to capture error contexts
        .init(); // Initialize the tracing subscriber
    Ok(())
}

pub fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

pub fn make_span_with_request_id(request: &Request<Body>) -> Span {
    let request_id = Uuid::new_v4();
    tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method = tracing::field::display(request.method()),
        uri = tracing::field::display(request.uri()),
        version = tracing::field::debug(request.version()),
        request_id = tracing::field::display(request_id),
    )
}

pub fn on_request(_request: &Request<Body>, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.as_u16();
    let status_code_class = status_code / 100;

    match status_code_class {
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
        _ => {
            tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
    };
}
