use std::sync::Arc;

use super::get_table_source;
use crate::context::FactlyApiContext;
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use roapi::{
    api::{encode_record_batches, encode_type_from_hdr},
    error::ApiErrResp,
};
use tokio::sync::Mutex;

pub async fn graphql<H: FactlyApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiErrResp> {
    let query = std::str::from_utf8(&body).map_err(ApiErrResp::read_query)?;
    let table_name = query
        .split(|c| c == '{' || c == '(' || c == '}' || c == ')')
        .collect::<Vec<&str>>()[1]
        .trim();

    let mut context = ctx.lock().await;
    if !context.get_table(table_name).await {
        let table_source = get_table_source(table_name);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_graphql(query).await?;
    encode_record_batches(encode_type, &batches)
}
