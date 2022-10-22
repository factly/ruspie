use std::sync::Arc;

use crate::context::FactlyApiContext;
use axum::{
    body::Bytes,
    extract,
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};

use roapi::{
    api::{encode_record_batches, encode_type_from_hdr},
    error::ApiErrResp,
};
use tokio::sync::Mutex;
use super::get_table_source;

pub async fn sql<H: FactlyApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let query = std::str::from_utf8(&body).map_err(ApiErrResp::read_query)?;
    let idx = query
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .position(|&x| x.to_lowercase() == "from")
        .unwrap();
    let table_name = query.split(" ").collect::<Vec<&str>>()[idx + 1];
    if !context.get_table(table_name.split(".").collect::<Vec<&str>>()[0]).await {
        let table_source = get_table_source(table_name);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_sql(query).await?;

    encode_record_batches(encode_type, &batches)
}