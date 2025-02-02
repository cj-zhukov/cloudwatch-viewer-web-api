use aws_config::{retry::RetryConfig, BehaviorVersion, Region};
use aws_sdk_cloudwatchlogs::{config::Builder, Client};

use super::constants::prod::AWS_MAX_RETRIES;

pub async fn get_aws_client(region: String) -> Client {
    let region = Region::new(region);

    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .load()
        .await;

    let config_builder = Builder::from(&sdk_config)
        .retry_config(RetryConfig::standard().with_max_attempts(AWS_MAX_RETRIES));

    let config = config_builder.build();

    Client::from_conf(config)
}
