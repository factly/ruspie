#![allow(dead_code)]
use mongodb::Client;

use super::Writer;

/// Writer to which fetched schemas will be stored
/// Mongo stores schemas in a MongoDB database: String arguments are the database and collection names
pub struct MongoWriter {
    client: Client,
    database: String,
    collection: String,
}

impl MongoWriter {
    /// Creates a new MongoWriter
    /// If no database or collection are provided, the default is "robinpie" and "schemas"
    pub fn new(client: Client, database: Option<String>, collection: Option<String>) -> Self {
        match (database, collection) {
            (Some(database), Some(collection)) => MongoWriter {
                client,
                database,
                collection,
            },
            _ => MongoWriter {
                client,
                database: "robinpie".to_string(),
                collection: "schemas".to_string(),
            },
        }
    }
}

impl Writer for MongoWriter {
    fn write<'life0, 'async_trait>(
        &'life0 self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = anyhow::Result<()>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn source(&self) -> &crate::context::Source {
        todo!()
    }

    fn schema_fetch(&self) -> &super::fetcher::SchemaFetcher<object_store::aws::AmazonS3> {
        todo!()
    }
}
