use std::{collections::HashMap, sync::Arc};

use super::{encode_vec_record_batches, extract_ext_from_headers, get_table_source};
use crate::context::RuspieApiContext;
use axum::{extract, http::HeaderMap, response::IntoResponse, Extension};
use columnq::encoding;
use roapi::{api::encode_type_from_hdr, error::ApiErrResp};
use tokio::sync::Mutex;

pub async fn rest<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    extract::Path(table): extract::Path<String>,
    extract::Query(mut params): extract::Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    if !context.table_exists(&table).await {
        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source(&table, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }

    if let Some(limit) = params.get("limit") {
        let limit = limit.parse::<u64>().unwrap();
        let max_limit = std::env::var("ENV_MAX_LIMIT")
            .unwrap_or_else(|_| String::from("30"))
            .parse::<u64>()
            .unwrap();
        if limit > max_limit {
            params.insert(String::from("limit"), max_limit.to_string());
        }
    } else {
        let limit = std::env::var("ENV_LIMIT").unwrap_or_else(|_| String::from("10"));
        params.insert(String::from("limit"), limit);
    }

    let encode_type = encode_type_from_hdr(headers, encoding::ContentType::default());
    let batches = context.query_rest_table_ruspie(&table, &params).await?;
    encode_vec_record_batches(encode_type, batches)
}
