use std::sync::Arc;

use crate::context::RuspieApiContext;
use axum::{
    extract,
    response::IntoResponse,
    Extension,
};
use roapi::{
    api::{bytes_to_json_resp},
    error::ApiErrResp,
};
use tokio::sync::Mutex;
use super::get_table_source;

pub async fn schema<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    extract::Path(table): extract::Path<String>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let table_name = table.split(".").collect::<Vec<&str>>()[0];
    println!("here 1");
    if !context.table_exists(table_name).await {
        println!("here 2");
        let table_source = get_table_source(&table);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let payload = context.table_schema_json_bytes(&table_name).await?;

    Ok(bytes_to_json_resp(payload))
}