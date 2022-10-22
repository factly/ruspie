use axum::routing::get;
use axum::Router;

use crate::context::FactlyApiContext;

use super::{graph::graphql, rest::{rest, rest_2}, schema::schema, sql::sql};

pub fn register_app_routes<H: FactlyApiContext>() -> Router {
    Router::new()
        .route("/api/tables/:table", get(rest::<H>))
        .route("/api/tables_2/:table", get(rest_2::<H>))
        .route("/api/graphql", get(graphql::<H>))
        .route("/api/sql", get(sql::<H>))
        .route("/api/schema/:table_name", get(schema::<H>))
}
