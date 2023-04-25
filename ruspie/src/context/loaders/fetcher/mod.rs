#![allow(dead_code)]

use crate::context::Schema;
pub mod mongo;

/// FileType is the type of file to be fetched where schemas are stored.
/// Defaulted to Json
#[derive(Debug, Default)]
pub enum FileType {
    #[default]
    Json,
    Parquet,
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
