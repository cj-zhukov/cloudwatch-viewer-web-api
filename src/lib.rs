pub mod app_state;
pub mod error;
pub mod logging_table;
pub mod routes;
pub mod utils;

use crate::app_state::AppState;
use crate::routes::*;

use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use error::ApiError;
use tokio::net::TcpListener;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(address: &str, app_state: AppState) -> Result<Self, ApiError> {
        let router = Router::new()
            .route("/", get(|| async { "CloudWatchViewer API" }))
            .route("/alive", get(ping))
            .route("/query", post(post_query))
            .with_state(app_state);

        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| ApiError::UnexpectedError(e.into()))?;
        let address = listener
            .local_addr()
            .map_err(|e| ApiError::UnexpectedError(e.into()))?
            .to_string();
        let server = axum::serve(listener, router);
        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<(), ApiError> {
        tracing::info!("listening on {}", &self.address);
        self.server
            .await
            .map_err(|e| ApiError::UnexpectedError(e.into()))?;
        Ok(())
    }
}
