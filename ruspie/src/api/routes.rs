use axum::routing::{delete, get, post};
use axum::Router;

use crate::context::api_context::RuspieApiContext;

use super::auth::handlers::{
    create_api_key, delete_api_key, get_api_keys, invalidate_key, update_api_key,
};
use super::{graph::graphql, rest::rest, schema::schema, sql::sql};

pub fn register_app_routes<H: RuspieApiContext>() -> Router {
    Router::new()
        .route("/api/tables/:table", get(rest::<H>))
        .route("/api/graphql", post(graphql::<H>))
        .route("/api/sql", post(sql::<H>))
        .route("/api/schema/:table_name", get(schema::<H>))
}

pub fn register_auth_api_routes() -> Router {
    Router::new()
        .route("/keys", post(create_api_key).get(get_api_keys))
        .route(
            "/keys/:key_id",
            delete(delete_api_key).patch(update_api_key),
        )
        .route("/keys/invalidate/:key_id", post(invalidate_key))
}
