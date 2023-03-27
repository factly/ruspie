pub mod logger;
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::Arc,
};

use axum::{http::Method, Extension, Router};
use roapi::server::http::HttpApiServer;
use tokio::sync::Mutex;

use crate::{
    api::{self, auth::middleware::auth_middleware, check_ext_middleware},
    context::{api_context::RuspieApiContext, auth::context::RawAuthContext},
};

pub fn build_http_server<H: RuspieApiContext>(
    ctx: Arc<Mutex<H>>,
    default_host: String,
    default_port: String,
) -> anyhow::Result<(HttpApiServer, SocketAddr)> {
    let http_addr = [default_host, default_port].join(":");

    let master_key = match std::env::var("MASTER_KEY") {
        Ok(key) => Some(key),
        Err(_) => None,
    };
    let auth = RawAuthContext::new(&Path::new("./"), &master_key)?;

    let routes: Router = api::routes::register_app_routes::<H>();
    let auth_routes = api::routes::register_auth_api_routes();
    // check the extension passed
    let middleware = axum::middleware::from_fn(move |req, next| check_ext_middleware(req, next));

    let mut app = routes.layer(Extension(ctx)).layer(middleware);

    if log::log_enabled!(log::Level::Info) {
        app = app.layer(logger::HttpLoggerLayer::new());
    }
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
        .allow_headers(tower_http::cors::Any)
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
