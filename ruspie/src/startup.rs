use crate::context::api_context::{RawRuspieApiContext, RuspieApiContext, Source};
use crate::server::build_http_server;
use columnq::table::{TableLoadOption, TableSchema, TableSource};
use log::{error, info};
use roapi::server::http::HttpApiServer;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

pub struct Application {
    pub http_addr: std::net::SocketAddr,
    pub http_server: HttpApiServer,
    pub table_reloader: TableReloader<RawRuspieApiContext>,
}

pub struct TableReloader<H: RuspieApiContext> {
    interval: std::time::Duration,
    ctx: Arc<Mutex<H>>,
    schema: Schemas,
}

#[derive(serde::Deserialize)]
pub struct Schemas {
    tables: Vec<Schema>,
}

#[derive(serde::Deserialize)]
pub struct Schema {
    name: String,
    extension: String,
    schema: TableSchema,
}

impl<H: RuspieApiContext> TableReloader<H> {
    pub async fn run(mut self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        let mut schema_file = tokio::fs::File::open("./schemas.json").await?;
        let mut buffer = String::new();
        schema_file.read_to_string(&mut buffer).await?;
        self.schema = serde_json::from_str(&buffer.trim()).unwrap();
        loop {
            interval.tick().await;
            for Schema {
                name,
                extension,
                schema,
            } in self.schema.tables.iter()
            {
                let mut map = serde_json::Map::new();
                map.insert(
                    String::from("format"),
                    serde_json::Value::String(extension.to_owned()),
                );
                map.insert(
                    String::from("use_memory_table"),
                    serde_json::Value::Bool(false),
                );
                let opt: TableLoadOption =
                    serde_json::from_value(serde_json::Value::Object(map)).unwrap();

                let path = std::env::var("S3_PATH").unwrap_or_else(|_| String::from("ruspie/"));
                let path = format!("s3://{}/{}.{}", path, name, extension);
                let source = TableSource::new(name, path)
                    .with_option(opt)
                    .with_schema(schema.clone());
                let mut ctx = self.ctx.lock().await;
                match ctx.conf_table(&source).await {
                    Ok(_) => info!("🚀 TableReloader reloaded schema of table {}", name),
                    Err(e) => error!("failed to reload schema for {:?}", e),
                }
            }
        }
    }
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        // let default_host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let default_host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let default_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let ctx = Arc::new(Mutex::new(RawRuspieApiContext::new()));
        let (http_server, http_addr) = build_http_server(ctx.clone(), default_host, default_port)?;
        let table_reloader = TableReloader {
            interval: std::time::Duration::from_secs(15),
            ctx,
            schema: Schemas { tables: vec![] },
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
        tokio::spawn(async move {
            println!("🚀 TableReloader spawned...");
            let _ = self.table_reloader.run().await;
        });
        self.http_server.await?;
        Ok(())
    }
}
