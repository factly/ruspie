use std::{collections::HashMap, sync::Arc};

use crate::context::FactlyApiContext;
use axum::{
    extract,
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};
use columnq::arrow::record_batch::RecordBatch;
use roapi::{
    api::{encode_record_batches, encode_type_from_hdr},
    error::ApiErrResp,
};
use tokio::sync::Mutex;
use super::get_table_source;

pub async fn rest<H: FactlyApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let table_name = &table.split(".").collect::<Vec<&str>>()[0];
    if !context.get_table(table_name).await {
        let table_source = get_table_source(&table);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_rest_table(table_name, &params).await?;
    encode_record_batches(encode_type, &batches)
}

pub async fn rest_2<H: FactlyApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let table_name = &table.split(".").collect::<Vec<&str>>()[0];
    if !context.get_table(table_name).await {
        let table_source = get_table_source(&table);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers);
    let batches = context.query_rest_table_2(table_name, &params).await?;
    let mut v: Vec<RecordBatch> = vec![];
    for mut v1 in batches {
        v.append(&mut v1)
    }
    encode_record_batches(encode_type, &v)
}