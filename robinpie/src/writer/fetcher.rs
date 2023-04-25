#![allow(dead_code)]
use futures::stream::StreamExt;
use std::sync::Arc;

use super::TableItem;

/// SchemaFetcher
/// Fetches list of list of file from S3 bucket
pub struct SchemaFetcher<H: object_store::ObjectStore> {
    object_store: Arc<H>,
}

impl<H: object_store::ObjectStore> SchemaFetcher<H> {
    /// Creates a new SchemaFetcher
    /// Takes an object store as an argument (currently only S3 is supported and AmazonS3)
    /// Only works with AmazonS3 object_store (S3 can be of any service)
    pub fn new(object_store: Arc<H>) -> Self {
        SchemaFetcher { object_store }
    }

    /// Fetches list of list of files from S3 bucket
    /// converts them to TableItem and returns a list of TableItem
    /// returns only files with extension parquet or csv
    pub async fn list_files(&self) -> anyhow::Result<Vec<TableItem>> {
        let mut list_stream = self.object_store.list(None).await?;
        let mut files: Vec<TableItem> = Vec::new();
        while let Some(file) = list_stream.next().await {
            let file = file.map_err(|e| {
                println!("{:?}", e);
                e
            })?;
            if file.location.extension() == Some("parquet")
                || file.location.extension() == Some("csv")
            {
                let mut table_item = TableItem::default();
                table_item.extension = file.location.extension().unwrap().to_string();
                table_item.name = file
                    .location
                    .filename()
                    .unwrap()
                    .to_string()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string();
                println!("{:?}", table_item);
                files.push(table_item);
            }
        }
        Ok(files)
    }
}
