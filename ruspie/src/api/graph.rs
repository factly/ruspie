use std::sync::Arc;

use super::{encode_vec_record_batches, extract_ext_from_headers, get_table_source};
use crate::context::{DatasetExtContext, RuspieApiContext};
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use roapi::{api::encode_type_from_hdr, error::ApiErrResp};
use tokio::sync::Mutex;

pub async fn graphql<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    Extension(dataset_ext_context): Extension<Arc<Mutex<DatasetExtContext>>>,
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
        let extension = extract_ext_from_headers(dataset_ext_context, &headers).await;
        let table_source = get_table_source(table_name, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_graphql_ruspie(query).await?;
    encode_vec_record_batches(encode_type, batches)
}
