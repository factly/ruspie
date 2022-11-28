use std::sync::Arc;

use super::{ extract_ext_from_headers, get_table_source};
use crate::context::RuspieApiContext;
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use roapi::{api::{encode_type_from_hdr, encode_record_batches}, error::ApiErrResp};
use tokio::sync::Mutex;

pub async fn graphql<H: RuspieApiContext>(
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
    if !context.table_exists(table_name).await {
        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source(table_name, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let response_format = H::get_response_format(&context).await;
    let encode_type = encode_type_from_hdr(headers, response_format);
    let batches = context.query_graphql(query).await?;
    encode_record_batches(encode_type, &batches)
}
