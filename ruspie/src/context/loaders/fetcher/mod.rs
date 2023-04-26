#![allow(dead_code)]

use crate::context::Schema;
pub mod mongo;
pub mod s3;

/// FileType is the type of file to be fetched where schemas are stored.
/// Defaulted to Json
#[derive(Debug, Default)]
pub enum FileType {
    #[default]
    Json,
    Parquet,
}

impl FileType {
    // returns the path to which fetched schemas will be stored
    pub fn s3_path(&self) -> url::Url {
        let extension = match self {
            FileType::Parquet => "parquet".to_string(),
            FileType::Json => "json".to_string(),
        };

        let bucket_name = std::env::var("S3_PATH").unwrap_or_else(|_| "ruspie".to_string());
        let path = format!("s3://{}/schemas.{}", bucket_name, extension);
        url::Url::parse(&path).unwrap()
    }
}

/// Source is the type of source where schemas are stored.
/// Defaulted to Mongo
/// Monge(String) String is the mongo_uri connection string
/// FileSystem(FileType) FileType is the type of file to be fetched where schemas are stored.
/// S3(FileType) FileType is the type of file to be fetched where schemas are stored.
pub enum Source {
    Mongo(String),
    FileSystem(FileType),
    S3(FileType),
}

/// Fetcher is the trait that all fetchers must implement.
/// Fetcher is responsible for fetching schemas from a source.
#[async_trait::async_trait]
pub trait Fetcher: Send + Sync + 'static {
    /// fetch is the method that fetches the schemas from a source.
    async fn fetch(&self) -> anyhow::Result<Vec<Schema>>;
}
