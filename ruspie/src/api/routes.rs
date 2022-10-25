use axum::routing::get;
use axum::Router;

use crate::context::RuspieApiContext;

use super::{graph::graphql, rest::{rest}, schema::schema, sql::sql};

pub fn register_app_routes<H: RuspieApiContext>() -> Router {
    Router::new()
        .route("/api/tables/:table", get(rest::<H>))
        .route("/api/graphql", get(graphql::<H>))
        .route("/api/sql", get(sql::<H>))
        .route("/api/schema/:table_name", get(schema::<H>))
}
