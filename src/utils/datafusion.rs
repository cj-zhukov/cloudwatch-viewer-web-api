use std::sync::Arc;

use crate::ClouWatchViewerError;

use datafusion::{
    arrow::datatypes::Schema, datasource::ViewTable, logical_expr::LogicalPlan,
    parquet::arrow::AsyncArrowWriter, prelude::*,
};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;

pub async fn register_logging_table(
    ctx: &SessionContext,
    plan: LogicalPlan,
    table_name: &str,
) -> Result<(), ClouWatchViewerError> {
    let view = ViewTable::try_new(plan, None)?;
    ctx.register_table(table_name, Arc::new(view))?;
    Ok(())
}

pub async fn write_df_to_file(
    df: DataFrame, 
    file_path: &str,
) -> Result<(), ClouWatchViewerError> {
    let mut buf = vec![];
    let schema = Schema::from(df.clone().schema());
    let mut stream = df.execute_stream().await?;
    let mut writer = AsyncArrowWriter::try_new(&mut buf, Arc::new(schema), None)?;
    while let Some(batch) = stream.next().await.transpose()? {
        writer.write(&batch).await?;
    }
    writer.close().await?;
    let mut file = File::create(file_path).await?;
    file.write_all(&buf).await?;
    Ok(())
}
