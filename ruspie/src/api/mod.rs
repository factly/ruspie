pub mod auth;
pub mod graph;
pub mod kavach;
pub mod rest;
pub mod routes;
pub mod schema;
pub mod sql;

use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use columnq::table::TableLoadOption;
use columnq::table::TableSource;

#[macro_export]
/// creates a new serde_json::Map and inserts format and use_memory_table values into it
macro_rules! create_serde_map {
    ($extension: expr) => {{
        let mut map = serde_json::Map::new();
        map.insert(
            String::from("format"),
            serde_json::Value::String($extension.to_owned()),
        );
        map
    }};
    ($extension: expr, $use_memory_table: ident) => {{
        let mut map = create_serde_map!($extension);
        map.insert(
            String::from("use_memory_table"),
            serde_json::Value::Bool(false),
        );
        map
    }};
}

pub fn get_table_source_fs(table_name: &str, extension: &str) -> TableSource {
    let map = create_serde_map!(extension, use_memory_table);
    let opt: TableLoadOption = serde_json::from_value(serde_json::Value::Object(map)).unwrap();
    let path = std::env::var("FILE_PATH").unwrap_or_else(|_| String::from("./data"));
    TableSource::new(
        table_name,
        format!(
            "./{}/{}",
            path,
            table_name.trim().to_owned() + "." + extension
        ),
    )
    .with_option(opt)
}

pub fn get_table_source_s3(tablename: &str, extension: &str, headers: &HeaderMap) -> TableSource {
    let map = create_serde_map!(extension, use_memory_table);
    let opt: TableLoadOption = serde_json::from_value(serde_json::Value::Object(map)).unwrap();

    let path: _ = if let Some(path) = headers
        .get("RUSPIE_BUCKET_NAME")
        .and_then(|path| path.to_str().ok())
    {
        path.to_owned()
    } else {
        std::env::var("RUSPIE_BUCKET_NAME").unwrap_or_else(|_| "ruspie".to_string())
    };
    let path = format!("s3://{}/{}.{}", path, tablename, extension);
    TableSource::new(tablename, path).with_option(opt)
}

pub fn extract_ext_from_headers(headers: &HeaderMap) -> String {
    let file_ext = headers.get("FILE-EXT");
    let extension: &str;

    let binding = std::env::var("DEFAULT_EXT").unwrap_or_else(|_| String::from("csv"));
    match file_ext {
        None => extension = binding.as_str(),
        Some(s) => extension = s.to_str().unwrap(),
    }

    extension.to_string()
}

pub fn get_limit() -> u64 {
    std::env::var("LIMIT")
        .unwrap_or_else(|_| "1000".to_owned())
        .parse::<u64>()
        .unwrap()
}

pub fn get_max_limit() -> u64 {
    std::env::var("MAX_LIMIT")
        .unwrap_or_else(|_| "1000".to_owned())
        .parse::<u64>()
        .unwrap()
}

pub async fn check_ext_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match req.headers().get("FILE-EXT") {
        Some(ext) => {
            if let Ok(ext) = ext.to_str() {
                if ext != "csv" && ext != "parquet" {
                    return Err(StatusCode::BAD_REQUEST);
                }
                return Ok(next.run(req).await);
            }
            Err(StatusCode::BAD_GATEWAY)
        }
        None => Ok(next.run(req).await),
    }
}
