pub mod utils;

use axum::{routing::get, serve::Serve, Router};
use axum::{http::StatusCode, response::IntoResponse};
use color_eyre::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }

    pub async fn build(address: &str) -> Result<Self> {        
        let router = Router::new()
            .route("/", get(|| async { "CloudWatchViewer App" }))
            .route("/alive", get(ping))
            .route("/logs", get(get_logs));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<()> {
        tracing::info!("listening on {}", &self.address);
        self.server.await?;
        Ok(())
    }
}

pub async fn ping() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn get_logs() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub fn init_tracing() -> Result<()> {
    let fmt_layer = fmt::layer().compact();
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer) 
        .with(fmt_layer) 
        .with(tracing_error::ErrorLayer::default()) 
        .init(); 

    Ok(())
}