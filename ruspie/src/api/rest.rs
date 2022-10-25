use std::{collections::HashMap, sync::Arc};

use crate::context::RuspieApiContext;
use axum::{
    extract,
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};
use roapi::{api::encode_type_from_hdr, error::ApiErrResp};
use tokio::sync::Mutex;
use super::{get_table_source, encode_vec_record_batches};

pub async fn rest<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let table_name = &table.split(".").collect::<Vec<&str>>()[0];
    if !context.table_exists(table_name).await {
        let table_source = get_table_source(&table);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_rest_table_ruspie(table_name, &params).await?;
    encode_vec_record_batches(encode_type, batches)
}