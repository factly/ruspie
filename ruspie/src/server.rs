use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::Arc,
};

use axum::{http::Method, Extension, Router};
use roapi::server::http::HttpApiServer;
use tokio::sync::Mutex;

use crate::{
    api::{self, auth::middleware::auth_middleware},
    context::{api_context::RuspieApiContext, auth::context::RawAuthContext},
};

pub fn build_http_server<H: RuspieApiContext>(
    ctx: Arc<Mutex<H>>,
    default_host: String,
) -> anyhow::Result<(HttpApiServer, SocketAddr)> {
    let default_http_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let http_addr = [default_host, default_http_port].join(":");

    let master_key = match std::env::var("MASTER_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    };
    let auth = RawAuthContext::new(&Path::new("./"), &master_key)?;

    let routes: Router = api::routes::register_app_routes::<H>();
    let auth_routes = api::routes::register_auth_api_routes();

    let mut app = routes.layer(Extension(ctx));

    let middleware =
        axum::middleware::from_fn(move |req, next| auth_middleware(req, next, auth.clone()));
    if master_key.is_some() {
        app = app
            .nest("/auth", auth_routes)
            .layer(middleware)
            .layer(Extension(RawAuthContext::new(
                &Path::new("./"),
                &master_key,
            )?));
    }

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(tower_http::cors::Any);

    app = app.layer(cors);
    let listener = TcpListener::bind(http_addr)?;
    let addr = listener
        .local_addr()
        .expect("Failed to get address from listener");
    let http_server = axum::Server::from_tcp(listener).unwrap();
    let http_server = http_server.serve(app.into_make_service());

    Ok((http_server, addr))
}
