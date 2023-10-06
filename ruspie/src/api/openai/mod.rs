mod text_to_sql;
use axum::{routing::post, Router};

use self::text_to_sql::text_to_sql;

pub fn register_text_to_sql_routes() -> Router {
    Router::new().route("/text_to_sql", post(text_to_sql))
}
