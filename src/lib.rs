pub mod app_state;
pub mod logging_table;
pub mod routes;
pub mod utils;

use crate::utils::constants::prod::LOGGING_TABLE_NAME;
use crate::routes::*;

use app_state::AppState;
use aws_sdk_cloudwatchlogs::Client;
use axum::{routing::{get, post}, serve::Serve, Router};
use color_eyre::Result;
use logging_table::LoggingTable;
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

    pub async fn build(address: &str, app_state: AppState) -> Result<Self> {        
        let router = Router::new()
            .route("/", get(|| async { "CloudWatchViewer App" }))
            .route("/alive", get(ping))
            .route("/query", post(post_query))
            .with_state(app_state);

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

pub async fn process_logging_table(client: Client, log_group_name: &str) -> Result<Vec<LoggingTable>> {
    let log_streams = client
        .describe_log_streams()
        .log_group_name(log_group_name)
        .send()
        .await?;

    let mut tasks = vec![];
    for log_stream in log_streams.log_streams() {
        if let Some(log_stream_name) = log_stream.log_stream_name() {
            let task = tokio::spawn(processs_log(
                client.clone(),
                log_group_name.to_string(),
                log_stream_name.to_string(),
                log_stream.creation_time,
                log_stream.first_event_timestamp,
                log_stream.last_event_timestamp,
                log_stream.last_ingestion_time,
                true,
            ));
            tasks.push(task);
        }
    }

    let mut records = vec![];
    for task in tasks {
        let logging_table = task.await??;
        records.extend(logging_table);
    }
    Ok(records)
}

async fn processs_log(
    client: Client,
    log_group_name: String,
    log_stream_name: String,
    log_creation_time: Option<i64>,
    first_event_timestamp: Option<i64>,
    last_event_timestamp: Option<i64>,
    last_ingestion_time: Option<i64>,
    start_from_head: bool,
) -> Result<Vec<LoggingTable>> {
    let log_events = client
        .get_log_events()
        .log_group_name(log_group_name)
        .log_stream_name(&log_stream_name)
        .start_from_head(start_from_head)
        .send()
        .await?;

    let mut res = vec![];
    for event in log_events.events() {
        let logging_table = LoggingTable::new(
            Some(log_stream_name.clone()),
            log_creation_time,
            first_event_timestamp,
            last_event_timestamp,
            last_ingestion_time,
            event.timestamp,
            event.message.clone(),
            event.ingestion_time,
        );
        res.push(logging_table);
    }
    Ok(res)
}
