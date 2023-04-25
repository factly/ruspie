use super::Fetcher;
use crate::context::Schema;
use futures::stream::TryStreamExt;

/// A fetcher that fetches data from a MongoDB collection.
/// It implements the Fetcher trait.
pub struct MongoFetcher {
    collection: mongodb::Collection<Schema>,
}

impl MongoFetcher {
    pub fn new(collection: mongodb::Collection<Schema>) -> Self {
        Self { collection }
    }
}

#[async_trait::async_trait]
impl Fetcher for MongoFetcher {
    async fn fetch(&self) -> anyhow::Result<Vec<Schema>> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut schemas = Vec::new();
        while let Some(result) = cursor.try_next().await.unwrap() {
            schemas.push(result);
        }
        Ok(schemas)
    }
}
