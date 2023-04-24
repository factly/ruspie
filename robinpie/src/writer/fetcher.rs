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
    pub fn new(object_store: Arc<H>) -> Self {
        SchemaFetcher { object_store }
    }

    pub async fn list_files(&self) -> anyhow::Result<Vec<TableItem>> {
        let mut list_stream = self.object_store.list(None).await?;
        let mut files: Vec<TableItem> = Vec::new();
        while let Some(file) = list_stream.next().await {
            let file = file.map_err(|e| anyhow::anyhow!(e))?;
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
                files.push(table_item);
            }
        }
        Ok(files)
    }
}
