#![allow(dead_code)]

use std::sync::Arc;

use mongodb::{options::ClientOptions, Client};
use object_store::aws::AmazonS3Builder;

use crate::writer::{mongo, s3::S3Writer, Writer};

/// File type to which fetched schemas will be stored (currently only Parquet and Json are supported)
/// Default is Json
#[derive(Debug, Default)]
pub enum FileType {
    Parquet,
    #[default]
    Json,
}

impl FileType {
    // returns the path to which fetched schemas will be stored
    pub fn s3_path(&self) -> url::Url {
        let extension = match self {
            FileType::Parquet => "parquet".to_string(),
            FileType::Json => "json".to_string(),
        };

        let bucket_name = std::env::var("BUCKET_NAME").unwrap_or_else(|_| "ruspie".to_string());
        let path = format!("s3://{}/schemas.{}", bucket_name, extension);
        url::Url::parse(&path).unwrap()
    }
}

/// Source to which fetched schemas will be stored
/// Mongo stores schemas in a MongoDB database (default): String argument is the MongoDB URI
/// FileSystem stores schemas in a local directory: FileType argument is the file type
/// S3 stores schemas in an S3 bucket: FileType argument is the file type
#[derive(Debug)]
pub enum Source {
    Mongo(String),
    FileSystem(FileType),
    S3(FileType),
}

impl Default for Source {
    fn default() -> Self {
        let mongo_uri: String = std::env::var("MONGO_URI")
            .unwrap_or("mongodb://mongo:mongo@localhost:27017".to_string());
        Source::Mongo(mongo_uri)
    }
}

/// Application Context
/// Contains information about the source to which fetched schemas will be stored
pub struct Application {
    source: Source,
    writer: Box<dyn Writer>,
}

impl Application {
    /// Creates a build Application Context
    /// If no source is provided, the default is a MongoDB database at localhost:27017
    pub async fn build() -> anyhow::Result<Self> {
        let source = std::env::var("PRE_FETCH_SOURCE").ok().map(|source| {
            match source.to_lowercase().as_str() {
                "mongo" => Source::Mongo(
                    std::env::var("MONGO_URI")
                        .unwrap_or("mongodb://mongo:mongo@localhost:27017".to_string()),
                ),
                "filesystem" => Source::FileSystem(FileType::default()),
                "s3" => Source::S3(FileType::default()),
                _ => Source::default(),
            }
        });

        let source = source.unwrap_or_default();
        let bucket_name = std::env::var("BUCKET_NAME").unwrap_or_else(|_| "ruspie".to_string());
        let object_store = Arc::new(
            // loads AWS credentials from environment variables
            AmazonS3Builder::from_env()
                .with_bucket_name(bucket_name)
                .with_allow_http(true)
                .build()
                .unwrap(),
        );
        // TODO: Add support for FileSystem
        let writer: Box<dyn Writer> = match &source {
            // Creates a new MongoWriter
            Source::Mongo(mongo_uri) => {
                let client_option = ClientOptions::parse(mongo_uri).await?;
                let client = Client::with_options(client_option)?;
                let database = std::env::var("MONGO_DATABASE").unwrap_or("robinpie".to_string());
                let collection = std::env::var("MONGO_COLLECTION").unwrap_or("schemas".to_string());

                let collection = client.database(&database).collection(&collection);

                Box::new(mongo::MongoWriter::new(collection, object_store))
            }
            Source::S3(FileType::Json) => Box::new(S3Writer::new(object_store, FileType::Json)),
            _ => unimplemented!(),
        };
        Ok(Self { source, writer })
    }

    /// Returns the source to which fetched schemas will be stored
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub async fn run_until_stopped(&self) -> anyhow::Result<()> {
        println!("🚀 Robinpie started from source {:?}...", self.source);
        let interval = std::env::var("ROBINPIE_PRE_FETCH_INTERVAL")
            .unwrap_or("30".to_string())
            .parse::<u64>()
            .unwrap_or(30);
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval));
        loop {
            interval.tick().await;
            println!("🚀 Robinpie starting to write schemas...");
            self.writer.write().await?;
        }
    }
}
