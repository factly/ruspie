use std::{collections::HashMap, sync::Arc};

use crate::context::{RuspieApiContext, DatasetExtContext};
use axum::{
    extract,
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};
use roapi::{api::encode_type_from_hdr, error::ApiErrResp};
use tokio::sync::Mutex;
use super::{get_table_source, encode_vec_record_batches, extract_ext_from_headers};

pub async fn rest<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    Extension(dataset_ext_context): extract::Extension<Arc<Mutex<DatasetExtContext>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    if !context.table_exists(&table).await {
        let extension = extract_ext_from_headers(dataset_ext_context, &headers).await;
        let table_source = get_table_source(&table, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_rest_table_ruspie(&table, &params).await?;
    encode_vec_record_batches(encode_type, batches)
}