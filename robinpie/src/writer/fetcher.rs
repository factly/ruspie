#![allow(dead_code)]
use futures::stream::StreamExt;
use object_store::path::Path;
use std::sync::Arc;

use super::{SchemaErrorResponse, SchemaResponse, TableItem};

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

    /// Fetches schema from ruspie
    /// Takes filename and extension as an argument
    /// Returns Option<TableItem>
    pub async fn fetch_from_ruspie(
        &self,
        filename: &str,
        extension: &str,
    ) -> anyhow::Result<Option<TableItem>> {
        // set headers
        let kavach_auth_token = std::env::var("KAVACH_AUTH_TOKEN");
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("FILE-EXT", extension.parse().unwrap());
        if let Ok(token) = kavach_auth_token {
            headers.insert(
                "Authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        // create new reqwest client
        let client = reqwest::Client::new();
        let url = std::env::var("RUSPIE_URL").unwrap_or_else(|_| "http://0.0.0.0:8080".to_string());
        let url = format!("{}/api/schema/{}", url, filename);

        println!("fetching from {}", url);
        // send request
        let response = client.get(&url).headers(headers).send().await?;

        if response.status() != 200 {
            // Deserialize the response body into an `SchemaErrorResponse`.
            let error = response.json::<SchemaErrorResponse>().await?;
            println!("{:?}", error);
            return Ok(None);
        }
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
        Ok(Some(table_item))
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
                files.push(table_item);
            }
        }
        Ok(files)
    }

    /// Pushes file to S3 bucket
    /// TODO: this is anti pattern, should be removed and moved to writer
    pub async fn push_file_to_s3(&self, location: Path, data: Vec<u8>) -> anyhow::Result<()> {
        self.object_store.put(&location, data.into()).await?;
        Ok(())
    }
}
