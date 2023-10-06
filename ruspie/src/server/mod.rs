pub mod logger;
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::Arc,
};

use axum::{http::Method, Extension, Router};
use log::info;
use roapi::server::http::HttpApiServer;
use tokio::sync::Mutex;

use crate::{
    api::{
        self,
        auth::middleware::auth_middleware,
        check_ext_middleware,
        kavach::{middleware::kavach_middleware, KavachValidateToken},
        openai::register_text_to_sql_routes,
    },
    context::{api_context::RuspieApiContext, auth::context::RawAuthContext, openai},
};

enum AuthType {
    Kavach,
    MasterKey(String),
    NoAuth,
}

impl From<String> for AuthType {
    fn from(value: String) -> Self {
        let master_key = std::env::var("MASTER_KEY").unwrap_or_else(|_| "".to_string());
        match value.to_owned().as_str() {
            "kavach" => AuthType::Kavach,
            "masterkey" => AuthType::MasterKey(master_key),
            _ => Self::NoAuth,
        }
    }
}

pub fn build_http_server<H: RuspieApiContext>(
    ctx: Arc<Mutex<H>>,
    default_host: String,
    default_port: String,
) -> anyhow::Result<(HttpApiServer, SocketAddr)> {
    let http_addr = [default_host, default_port].join(":");

    let routes: Router = api::routes::register_app_routes::<H>();
    // check the extension passed
    let middleware = axum::middleware::from_fn(move |req, next| check_ext_middleware(req, next));

    let mut app = routes.layer(Extension(ctx)).layer(middleware);

    if log::log_enabled!(log::Level::Info) {
        app = app.layer(logger::HttpLoggerLayer::new());
    }

    let auth_type: AuthType = std::env::var("AUTH_TYPE")
        .unwrap_or(String::from(""))
        .into();
    match auth_type {
        AuthType::MasterKey(master_key) => {
            let auth = RawAuthContext::new(&Path::new("./"), &Some(master_key.clone()))?;
            let auth_middleware = axum::middleware::from_fn(move |req, next| {
                auth_middleware(req, next, auth.clone())
            });
            let auth_routes = api::routes::register_auth_api_routes();
            app = app
                .nest("/auth", auth_routes)
                .layer(auth_middleware)
                .layer(Extension(RawAuthContext::new(
                    &Path::new("./"),
                    &Some(master_key),
                )?));
        }
        AuthType::Kavach => {
            let client = reqwest::Client::new();
            let auth = KavachValidateToken::new(client);
            let kavach_middleware = axum::middleware::from_fn(move |req, next| {
                kavach_middleware(req, next, auth.clone())
            });
            app = app.layer(kavach_middleware);
        }
        AuthType::NoAuth => {}
    }

    let text_to_sql_enable =
        std::env::var("TEXT_TO_SQL")
            .ok()
            .and_then(|e| match e.to_lowercase().as_str() {
                "true" => Some(true),
                _ => Some(false),
            });
    if let Some(enabled) = text_to_sql_enable {
        if enabled {
            info!("text_to_sql service is enabled");
            let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
            let endpoint_url =
                std::env::var("OPENAI_ENDPOINT_URL").expect("OPENAI_ENDPOINT_URL is not set");

            let openai_ctx = openai::OpenAIContext::new(api_key, endpoint_url);
            let text_to_sql_routes = register_text_to_sql_routes();
            app = app
                .nest("/", text_to_sql_routes)
                .layer(Extension(openai_ctx));
        }
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
