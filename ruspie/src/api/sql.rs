use std::sync::Arc;

use super::{extract_ext_from_headers, get_limit, get_table_source_fs, get_table_source_s3};
use crate::{
    api::get_max_limit,
    context::api_context::{RuspieApiContext, Source},
};
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use columnq::encoding;
use roapi::{
    api::{encode_record_batches, encode_type_from_hdr},
    error::ApiErrResp,
};
use sqlparser::{
    ast::{Statement, TableFactor},
    dialect::GenericDialect,
    parser::Parser,
};
use tokio::sync::Mutex;

pub async fn sql<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let query = std::str::from_utf8(&body)
        .map_err(ApiErrResp::read_query)?
        .to_string();
    let table_name = match extract_table_name(&query) {
        None => return Err(ApiErrResp::not_found("table not found/ invalid query")),
        Some(name) => name,
    };
    if !context.table_exists(&table_name).await {
        let extension = extract_ext_from_headers(&headers);
        // let table_source = get_table_source_fs(table_name, &extension);
        let table_source = match context.get_source() {
            &Source::S3 => get_table_source_s3(&table_name, &extension, &headers),
            _ => get_table_source_fs(&table_name, &extension),
        };
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let mut query = query.replace(";", "");

    if query.contains("limit") || query.contains("LIMIT") {
        let vec_q = query.split(" ").collect::<Vec<&str>>();
        let i = vec_q
            .iter()
            .position(|&x| x == "limit" || x == "LIMIT")
            .unwrap();
        let mut limit = vec_q[i + 1].parse::<u64>().unwrap();
        if limit > get_max_limit() {
            limit = get_limit();
        }
        query = query.replace(vec_q[i + 1], limit.to_string().as_str())
    } else {
        let limit = get_limit().to_string();
        query = query + " limit " + limit.as_str();
    }
    let encode_type = encode_type_from_hdr(headers, encoding::ContentType::default());
    let batches = context.query_sql_ruspie(query.as_str()).await?;

    encode_record_batches(encode_type, &batches)
}

fn extract_table_name(sql: &str) -> Option<String> {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql).ok()?;
    match ast.iter().next().unwrap() {
        Statement::Query(ref query) => {
            if let sqlparser::ast::SetExpr::Select(body) = &query.body {
                if body.from.len() < 1 {
                    return None;
                }
                if let TableFactor::Table { name, .. } = &body.from[0].relation {
                    if name.0.len() < 1 {
                        return None;
                    }
                    return Some(name.0[0].value.clone());
                }
            }
            return None;
        }
        _ => return None,
    }
}
