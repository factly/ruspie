#![allow(dead_code)]
use std::sync::Arc;

use mongodb::{
    bson::{doc, to_bson},
    options::UpdateModifications,
};
use object_store::aws::AmazonS3;

type Collection = mongodb::Collection<TableItem>;

use super::{fetcher::SchemaFetcher, TableItem, Writer};

/// Writer to which fetched schemas will be stored
/// Mongo stores schemas in a MongoDB database: String arguments are the database and collection names
pub struct MongoWriter<H: object_store::ObjectStore> {
    collection: Collection,
    fetcher: SchemaFetcher<H>,
}

impl<H: object_store::ObjectStore> MongoWriter<H> {
    /// Creates a new MongoWriter
    /// If no database or collection are provided, the default is "robinpie" and "schemas"
    pub fn new(collection: Collection, object_store: Arc<H>) -> Self {
        let fetcher = SchemaFetcher::new(object_store);
        MongoWriter {
            collection,
            fetcher,
        }
    }
}

#[async_trait::async_trait]
impl Writer for MongoWriter<AmazonS3> {
    #[inline]
    async fn write(&self) -> anyhow::Result<()> {
        // get list of files from S3
        let list_of_files = self.schema_fetcher().list_files().await?;

        // iter over list of files
        for file in list_of_files.iter() {
            let filter = doc! { "name": &file.name, "extension": &file.extension };
            let table_item = self
                .fetcher
                .fetch_from_ruspie(&file.name, &file.extension)
                .await?;
            if let Some(table_item) = table_item {
                let update = UpdateModifications::Document(doc! { "$set": to_bson(&table_item)? });
                let options = mongodb::options::UpdateOptions::builder()
                    .upsert(true)
                    .build();

                self.collection.update_one(filter, update, options).await?;
            }
            // self.collection.insert_one(table_item, None).await?;
        }
        log::info!("writing schemas writing to successful");
        Ok(())
    }

    #[inline]
    fn schema_fetcher(&self) -> &super::fetcher::SchemaFetcher<object_store::aws::AmazonS3> {
        &self.fetcher
    }
}
