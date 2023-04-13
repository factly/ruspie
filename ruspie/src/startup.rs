use crate::context::api_context::{RawRuspieApiContext, Source};
use crate::context::loaders::schema::{S3FileSchemaLoader, SchemaFileType};
use crate::context::loaders::table::TableReloader;
use crate::context::Schemas;
use crate::server::build_http_server;
use roapi::server::http::HttpApiServer;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Application {
    pub http_addr: std::net::SocketAddr,
    pub http_server: HttpApiServer,
    pub table_reloader: Option<TableReloader<RawRuspieApiContext>>,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let default_host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let default_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let enable_prefetch =
            std::env::var("ENABLE_PREFETCH").unwrap_or_else(|_| false.to_string());
        let ctx = Arc::new(Mutex::new(RawRuspieApiContext::new()));
        let (http_server, http_addr) = build_http_server(ctx.clone(), default_host, default_port)?;
        let loader = S3FileSchemaLoader::new("schemas".to_string(), SchemaFileType::Json);
        let table_reloader = match enable_prefetch.as_str() {
            "true" => Some(TableReloader {
                interval: std::time::Duration::from_secs(60),
                ctx,
                loader,
                schemas: Schemas { tables: vec![] },
            }),
            "false" => None,
            _ => panic!("invalid value for ENABLE_PREFETCH (should be true or false)"),
        };

        Ok(Self {
            http_addr,
            http_server,
            table_reloader,
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

        if let Some(table_reloader) = self.table_reloader {
            tokio::spawn(async move {
                println!("🚀 TableReloader spawned...");
                let _ = table_reloader.run().await;
            });
        }

        self.http_server.await?;
        Ok(())
    }
}
