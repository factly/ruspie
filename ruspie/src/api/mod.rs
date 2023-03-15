pub mod auth;
#[allow(dead_code)]
pub mod graph;
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

pub fn get_table_source_fs(table_name: &str, extension: &str) -> TableSource {
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
    let path = std::env::var("FILE_PATH").unwrap_or_else(|_| String::from("test"));
    TableSource::new(
        table_name,
        format!(
            "./{}/{}",
            path,
            table_name.clone().trim().to_owned() + "." + extension
        ),
    )
    .with_option(opt)
}

pub fn get_table_source_s3(path: &str, extension: &str, headers: &HeaderMap) -> TableSource {
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
    let path = "s3://".to_owned() + headers.get("PATH").unwrap().to_str().unwrap() + path;
    let table_name = {
        let path: &Vec<&str> = &path.split("/").collect::<Vec<&str>>();
        path[path.len() - 1]
    };
    TableSource::new(table_name, path.to_owned() + "." + extension).with_option(opt)
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

pub fn get_limit() -> i64 {
    std::env::var("LIMIT")
        .unwrap_or_else(|_| "30".to_owned())
        .parse::<i64>()
        .unwrap()
}

pub fn get_max_limit() -> i64 {
    std::env::var("MAX_LIMIT")
        .unwrap_or_else(|_| "40".to_owned())
        .parse::<i64>()
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
