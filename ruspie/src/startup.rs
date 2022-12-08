use roapi::server::http::HttpApiServer;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{context::api_context::RawRuspieApiContext, server};

pub struct Application {
    pub http_addr: std::net::SocketAddr,
    pub http_server: HttpApiServer,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let default_host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let ctx = Arc::new(Mutex::new(RawRuspieApiContext::new()));
        let (http_server, http_addr) =
            server::build_http_server(ctx, default_host)?;

        Ok(Self {
            http_addr,
            http_server,
        })
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        println!("🚀 Listening on {} for HTTP traffic...", self.http_addr);
        Ok(self.http_server.await?)
    }
}
