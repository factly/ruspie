use std::{collections::HashMap, sync::Arc};

use crate::context::RuspieApiContext;
use axum::{
    extract,
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};
use roapi::{api::{encode_type_from_hdr, encode_record_batches}, error::ApiErrResp};
use tokio::sync::Mutex;
use super::{get_table_source, extract_ext_from_headers};

pub async fn rest<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    println!("called");
    let mut context = ctx.lock().await;
    if !context.table_exists(&table).await {
        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source(&table, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let response_format = H::get_response_format(&context).await;
    let encode_type = encode_type_from_hdr(headers, response_format);
    let batches = context.query_rest_table(&table, &params).await?;
    encode_record_batches(encode_type, &batches)
}