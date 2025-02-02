use cloudwatch_viewer_web_api::{app_state::AppState, init_tracing, logging_table::LoggingTable, process_logging_table, utils::{aws::get_aws_client, constants::{prod::{self, LOGGING_TABLE_NAME, REGION}, LOG_GROUP_NAME_SECRET}, datafusion::register_logging_table}, Application};

use color_eyre::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    let ctx = SessionContext::new();
    let client = get_aws_client(REGION.to_string()).await;
    let records = process_logging_table(client.clone(), &LOG_GROUP_NAME_SECRET).await?;
    let df = LoggingTable::to_df(&ctx, &records).await?;
    register_logging_table(&ctx, df.logical_plan().clone(), LOGGING_TABLE_NAME).await?;
    let app_state = AppState::new(ctx, client);

    let app = Application::build(prod::APP_ADDRESS, app_state).await?;
    app.run().await?;   
 
    Ok(())
}
