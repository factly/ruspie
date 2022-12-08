use axum::routing::{get,post, delete, patch};
use axum::Router;

use crate::context::api_context::RuspieApiContext;

use super::auth::handlers::{create_api_key, get_api_keys, delete_api_key, update_api_key};
use super::{graph::graphql, rest::{rest}, schema::schema, sql::sql};

pub fn register_app_routes<H: RuspieApiContext>() -> Router {
    Router::new()
        .route("/api/tables/:table", get(rest::<H>))
        .route("/api/graphql", post(graphql::<H>))
        .route("/api/sql", post(sql::<H>))
        .route("/api/schema/:table_name", get(schema::<H>))
        .route("/keys/create", post(create_api_key))
        .route("/keys", get(get_api_keys))
        .route("/keys/:key_id", delete(delete_api_key))
        .route("/keys/:key_id", patch(update_api_key))
}
