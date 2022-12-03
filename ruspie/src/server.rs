use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use axum::{http::Method, Extension, Router};
use roapi::server::http::HttpApiServer;
use tokio::sync::Mutex;

use crate::{api, context::RuspieApiContext};

pub fn build_http_server<H: RuspieApiContext>(
    ctx: Arc<Mutex<H>>,
    default_host: String,
) -> anyhow::Result<(HttpApiServer, SocketAddr)> {
    let default_http_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let http_addr = [default_host, default_http_port].join(":");
    let routes: Router = api::routes::register_app_routes::<H>();
    let mut app = routes
        .layer(Extension(ctx));

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
