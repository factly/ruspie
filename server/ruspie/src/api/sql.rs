use std::sync::Arc;

use crate::context::RuspieApiContext;
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};

use super::{ get_table_source, extract_ext_from_headers};
use roapi::{api::{encode_type_from_hdr, encode_record_batches}, error::ApiErrResp};
use tokio::sync::Mutex;

pub async fn sql<H: RuspieApiContext>(
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
    if !context
        .table_exists(table_name)
        .await
    {

        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source(table_name, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let response_format = H::get_response_format(&context).await;
    let encode_type = encode_type_from_hdr(headers, response_format);
    let batches = context.query_sql(query).await?;

    // encode_vec_record_batches(encode_type, batches)
    encode_record_batches(encode_type, &batches)
}
