use std::io::Error as IoError;

use aws_sdk_cloudwatchlogs::error::SdkError;
use aws_sdk_cloudwatchlogs::operation::describe_log_streams::DescribeLogStreamsError;
use aws_sdk_cloudwatchlogs::operation::get_log_events::GetLogEventsError;
use color_eyre::eyre::Report;
use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError;
use datafusion::parquet::errors::ParquetError;
use thiserror::Error;
use tokio::task::JoinError;
use tracing_subscriber::filter::FromEnvError;
use tracing_subscriber::filter::ParseError;

#[derive(Debug, Error)]
pub enum LoggingTableError {
    #[error("Arrow error")]
    ArrowError(#[from] ArrowError),

    #[error("AWS DescribeLogStreams error")]
    DescribeLogStreamsError(#[from] SdkError<DescribeLogStreamsError>),

    #[error("AWS GetLogEventsError error")]
    GetLogEventsError(#[from] SdkError<GetLogEventsError>),

    #[error("DataFusion error")]
    DataFusionError(#[from] DataFusionError),

    #[error("IO error")]
    IoError(#[from] IoError),

    #[error("Parquet error")]
    ParquetError(#[from] ParquetError),

    #[error("TokioJoin error")]
    TokioJoinError(#[from] JoinError),

    #[error("Tracing FromEnv error")]
    FromEnvError(#[from] FromEnvError),

    #[error("Tracing Parse error")]
    ParseError(#[from] ParseError),

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
