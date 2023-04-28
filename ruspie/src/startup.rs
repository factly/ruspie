use crate::context::api_context::{RawRuspieApiContext, Source};
use crate::context::loaders::fetcher::mongo::MongoFetcher;
use crate::context::loaders::fetcher::s3::S3Fetcher;
use crate::context::loaders::fetcher::Source as PrefetchSource;
use crate::context::loaders::fetcher::{Fetcher, FileType};
use crate::context::loaders::table::TableReloader;
use crate::context::Schemas;
use crate::server::build_http_server;
use mongodb::options::ClientOptions;
use mongodb::Client;
use object_store::aws::AmazonS3Builder;
use roapi::server::http::HttpApiServer;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Application {
    pub http_addr: std::net::SocketAddr,
    pub http_server: HttpApiServer,
    pub table_reloader: Option<TableReloader<RawRuspieApiContext>>,
}

async fn get_fetcher() -> Option<Box<dyn Fetcher>> {
    let enable_prefetch = std::env::var("PRE_FETCH_ENABLED").unwrap_or_else(|_| false.to_string());
    if enable_prefetch == "false" {
        return None;
    }
    let source = std::env::var("PRE_FETCH_SOURCE").unwrap_or_else(|_| "mongo".to_string());
    let filetype = FileType::default();
    let source = match source.to_lowercase().as_str() {
        "s3" => PrefetchSource::S3(filetype),
        "mongo" => {
            let mongo_uri = std::env::var("MONGO_URI")
                .unwrap_or("mongodb://mongo:mongo@localhost:27017".to_string());
            PrefetchSource::Mongo(mongo_uri)
        }
        "filesystem" => PrefetchSource::FileSystem(filetype),
        _ => panic!("invalid value for PRE_FETCH_SOURCE (should be mongo, s3 or filesystem)"),
    };

    match source {
        PrefetchSource::S3(filetype) => {
            let bucket_name = std::env::var("S3_PATH").unwrap_or_else(|_| "ruspie".to_string());
            let object_store = Arc::new(
                // loads AWS credentials from environment variables
                AmazonS3Builder::from_env()
                    .with_bucket_name(bucket_name)
                    .with_allow_http(true)
                    .build()
                    .unwrap(),
            );
            Some(Box::new(S3Fetcher::new(object_store, filetype)))
        }
        PrefetchSource::Mongo(mongo_uri) => {
            let client_option = ClientOptions::parse(mongo_uri).await.unwrap();
            let client = Client::with_options(client_option).unwrap();
            let database = std::env::var("MONGO_DATABASE").unwrap_or("robinpie".to_string());
            let collection = std::env::var("MONGO_COLLECTION").unwrap_or("schemas".to_string());

            let collection = client.database(&database).collection(&collection);

            Some(Box::new(MongoFetcher::new(collection)))
        }
        PrefetchSource::FileSystem(_) => todo!(),
    }
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let default_host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let default_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let ctx = Arc::new(Mutex::new(RawRuspieApiContext::new()));
        let (http_server, http_addr) = build_http_server(ctx.clone(), default_host, default_port)?;
        let table_reloader = get_fetcher().await.and_then(|fetcher| {
            let interval = std::env::var("RUPSIE_PRE_FETCH_INTERVAL")
                .unwrap_or_else(|_| "60".to_string())
                .parse::<u64>()
                .unwrap();
            Some(TableReloader {
                interval: std::time::Duration::from_secs(interval),
                ctx,
                fetcher,
                schemas: Schemas { tables: vec![] },
            })
        });
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
