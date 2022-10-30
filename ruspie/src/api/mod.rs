pub mod graph;
pub mod rest;
pub mod routes;
pub mod schema;
pub mod sql;

use std::sync::Arc;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use columnq::datafusion::arrow;
use columnq::encoding::ContentType;
use columnq::table::TableLoadOption;
use columnq::table::TableSource;
use roapi::api::encode_record_batches;
use roapi::error::ApiErrResp;
use tokio::sync::Mutex;

use crate::context::DatasetExtContext;

pub fn get_table_source(table_name: &str, extension: &str) -> TableSource {
    let mut map = serde_json::Map::new();
    map.insert(
        String::from("format"),
        serde_json::Value::String(extension.to_owned()),
    );
    map.insert(
        String::from("use_memory_table"),
        serde_json::Value::Bool(false),
    );
    let opt: TableLoadOption = serde_json::from_value(serde_json::Value::Object(map)).unwrap();
    TableSource::new(
        table_name,
        format!(
            "./test/{}",
            table_name.clone().trim().to_owned() + "." + extension
        ),
    )
    .with_option(opt)
}

pub fn encode_vec_record_batches(
    content_type: ContentType,
    batches: Vec<Vec<arrow::record_batch::RecordBatch>>,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut v = vec![];
    for mut v1 in batches {
        v.append(&mut v1)
    }
    encode_record_batches(content_type, &v)
}

pub async fn extract_ext_from_headers(dataset_ext_context: Arc<Mutex<DatasetExtContext>>, headers: &HeaderMap) -> String {
    let mut dsectx = dataset_ext_context.lock().await;
        let default_ext = headers.get("default_ext");
        let current_ext = headers.get("current_ext");
        let extension: &str;

        if let Some(v) = default_ext {
            dsectx.set_default_ext(v.to_str().unwrap().to_string());
        }
        if let Some(v) = current_ext {
            dsectx.set_current_ext(Some(v.to_str().unwrap().to_string()));
            extension = v.to_str().unwrap();
        } else {
            let ex = dsectx.get_default_ext();
            extension = ex.as_str();
        }

        extension.to_string()
}