use roapi::server::http::HttpApiServer;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::context::api_context::{RawRuspieApiContext, Source};
use crate::server::build_http_server;

pub struct Application {
    pub http_addr: std::net::SocketAddr,
    pub http_server: HttpApiServer,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let default_host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let ctx = Arc::new(Mutex::new(RawRuspieApiContext::new()));
        let (http_server, http_addr) = build_http_server(ctx, default_host)?;

        Ok(Self {
            http_addr,
            http_server,
        })
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        let source: Source = std::env::var("SOURCE")
            .unwrap_or_else(|_| "FILESYSTEM".to_string())
            .into();
        println!(
            "🚀 Listening on {} for HTTP traffic from file source `{:?}`...",
            self.http_addr, source
        );
        Ok(self.http_server.await?)
    }
}
