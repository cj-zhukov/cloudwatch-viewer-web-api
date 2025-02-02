use cloudwatch_viewer_web_api::{init_tracing, utils::constants::prod, Application};

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing()?;

    let app = Application::build(prod::APP_ADDRESS).await?;
    app.run().await?;   
    
    Ok(())
}
