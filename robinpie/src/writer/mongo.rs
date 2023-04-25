#![allow(dead_code)]
use std::sync::Arc;

use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use object_store::aws::AmazonS3;

type Collection = mongodb::Collection<TableItem>;

use super::{fetcher::SchemaFetcher, SchemaResponse, TableItem, Writer};

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
    async fn fetch_from_ruspie(
        &self,
        filename: &str,
        extension: &str,
    ) -> anyhow::Result<TableItem> {
        // set headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("FILE-EXTENSION", extension.parse().unwrap());

        // create new reqwest client
        let client = reqwest::Client::new();
        let url = std::env::var("RUSPIE_URL").unwrap_or_else(|_| "http://0.0.0.0:8080".to_string());
        let url = format!("{}/api/schema/{}", url, filename);

        // send request
        let response = client.get(&url).headers(headers).send().await?;

        // parse response to TableItem
        let table_item = response
            .json()
            .await
            .and_then(|schema_resp: SchemaResponse| {
                Ok(TableItem {
                    name: filename.to_string(),
                    extension: extension.to_string(),
                    schema: schema_resp.into(),
                })
            })?;
        Ok(table_item)
    }

    #[inline]
    async fn write(&self) -> anyhow::Result<()> {
        // get list of files from S3
        let list_of_files = self.schema_fetcher().list_files().await?;

        // iter over list of files
        // if file is not in database, fetch schema from ruspie and insert into database
        for file in list_of_files.iter() {
            let filter = doc! { "name": &file.name, "extension": &file.extension };
            let mut result = self.collection.find(filter, None).await?;
            // TODO: change this to a more efficient way of checking if the file is in the database
            let mut list = vec![];
            while let Some(e) = result.try_next().await? {
                list.push(e)
            }
            if list.len() == 0 {
                let table_item = self.fetch_from_ruspie(&file.name, &file.extension).await?;
                self.collection.insert_one(table_item, None).await?;
            }
        }
        Ok(())
    }

    #[inline]
    fn schema_fetcher(&self) -> &super::fetcher::SchemaFetcher<object_store::aws::AmazonS3> {
        &self.fetcher
    }
}
