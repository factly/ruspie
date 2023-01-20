use std::sync::Arc;

use super::get_max_limit;
use super::get_limit;
use super::{extract_ext_from_headers, get_table_source};
use crate::context::api_context::RuspieApiContext;
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use columnq::encoding;
use columnq::error::QueryError;
use roapi::api::encode_record_batches;
use roapi::{api::encode_type_from_hdr, error::ApiErrResp};
use tokio::sync::Mutex;
use graphql_parser::query::Definition;
use graphql_parser::query::OperationDefinition;
use graphql_parser::query::Selection;
use graphql_parser::schema::Value;

pub async fn graphql<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiErrResp> {
    let query = std::str::from_utf8(&body).map_err(ApiErrResp::read_query)?;
    let (q, table_name) = parse_graphql_query(query)?;
    let mut context = ctx.lock().await;
    if !context.table_exists(table_name.as_str()).await {
        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source(table_name.as_str(), &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let encode_type = encode_type_from_hdr(headers, encoding::ContentType::default());
    let batches = context.query_graphql_ruspie(q.as_str()).await?;
    encode_record_batches(encode_type, &batches)
}

pub fn parse_graphql_query(query: &str) -> Result<(String, String), QueryError> {
    let doc = graphql_parser::parse_query::<&str>(query)?;

    let def = match doc.definitions.len() {
        1 => match &doc.definitions[0] {
            Definition::Operation(op_def) => op_def,
            Definition::Fragment(_) => {
                return Err(QueryError {
                    error: "invalid graphql query".to_string(),
                    message: "TODO: fragment definition not supported, please file a Github issue"
                        .to_string(),
                });
            }
        },
        0 => {
            return Err(QueryError {
                error: "invalid graphql query".to_string(),
                message: "empty query".to_string(),
            });
        }
        n => {
            return Err(QueryError {
                error: "invalid graphql query".to_string(),
                message: format!("only 1 definition allowed, got: {}", n),
            });
        }
    };

    let selections = &match def {
        OperationDefinition::Query(query) => &query.selection_set,
        OperationDefinition::SelectionSet(sel) => sel,
        _ => {
            return Err(QueryError {
                error: "invalid graphql query".to_string(),
                message: format!("Unsupported operation: {}", def),
            });
        }
    }
    .items;

    let mut field = None;
    // TODO: reenable clippy::never_loop rule after we added support for FragmentSpread and
    // InlineFragment
    #[allow(clippy::never_loop)]
    for selection in selections {
        match selection {
            Selection::Field(f) => {
                field = Some(f);
                break;
            }
            Selection::FragmentSpread(_) => {
                return Err(QueryError {
                    error: "invalid graphql query".to_string(),
                    message:
                        "TODO: Selection::FragmentSpread not supported, please file github issue"
                            .to_string(),
                });
            }
            Selection::InlineFragment(_) => {
                return Err(QueryError {
                    error: "invalid graphql query".to_string(),
                    message:
                        "TODO: Selection::InlineFragment not supported, please file github issue"
                            .to_string(),
                });
            }
        }
    }

    let field = field.ok_or_else(|| invalid_query("field not found in selection".to_string()))?;

    let mut filter: Option<&Value<&str>> = None;
    let mut sort: Option<&Value<&str>> = None;
    let mut limit: Option<&Value<&str>> = None;
    let mut page: Option<&Value<&str>> = None;
    for (key, value) in &field.arguments {
        match *key {
            "filter" => {
                filter = Some(value);
            }
            "sort" => {
                sort = Some(value);
            }
            "limit" => {
                limit = Some(value);
            }
            "page" => page = Some(value),
            other => {
                return Err(invalid_query(format!("invalid query argument: {}", other)));
            }
        }
    }

    let mut query = "query {".to_owned() + field.name + "(";

    let column_names = field
        .selection_set
        .items
        .iter()
        .map(|selection| match selection {
            Selection::Field(f) => Ok(f.name),
            _ => Err(QueryError {
                error: "invalid graphql query".to_string(),
                message: "selection set in query should only contain Fields".to_string(),
            }),
        })
        .collect::<Result<Vec<&str>, _>>()?;
    if let Some(limit) = limit {
        let mut limit = limit.to_string().parse::<i64>().unwrap();
        let max_limit = get_max_limit();
        if limit > max_limit{
            limit = get_limit();
        }
        query = query.to_owned() + "limit:" + limit.to_string().as_str() + ",";
    }
    if let Some(filter) = filter {
        query = query.to_owned() + "filter:" + filter.to_string().as_str() + ",";
    }
    if let Some(sort) = sort {
        query = query.to_owned() + "sort:" + sort.to_string().as_str() + ",";
    }
    if let Some(page) = page {
        query = query.to_owned() + "page:" + page.to_string().as_str() + ","
    }
    query = query.to_owned() + ") {";
    for i in column_names {
        query = query.to_owned() + i + ",";
    }
    query = query.to_owned() + "}}";
    Ok((query, field.name.to_owned()))
}

fn invalid_query(message: String) -> QueryError {
    QueryError {
        error: "invalid graphql query".to_string(),
        message,
    }
}