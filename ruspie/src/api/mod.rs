pub mod graph;
pub mod rest;
pub mod routes;
pub mod schema;
pub mod sql;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use columnq::datafusion::arrow;
use columnq::encoding::ContentType;
use columnq::table::TableLoadOption;
use columnq::table::TableSource;
use roapi::api::encode_record_batches;
use roapi::error::ApiErrResp;

pub fn get_table_source(table_name: &str, extension: &str) -> TableSource {
    let mut map = serde_json::Map::new();
    map.insert(
        String::from("format"),
        serde_json::Value::String(extension.to_owned()),
    );
    map.insert(
        String::from("use_memory_table"),
        serde_json::Value::Bool(true),
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

pub fn extract_ext_from_headers(headers: &HeaderMap) -> String {
    let file_ext = headers.get("file-ext");
    let extension: &str;

    match file_ext {
        None => extension = "csv",
        Some(s) => extension = s.to_str().unwrap(),
    }

    extension.to_string()
}
