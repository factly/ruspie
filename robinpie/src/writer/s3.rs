#![allow(dead_code)]

use std::sync::Arc;

use object_store::{aws::AmazonS3, path::Path, ObjectStore};

use crate::context::FileType;

use super::{fetcher::SchemaFetcher, Writer};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
/// S3Writer is a writer that writes schemas to S3
/// It implements the Writer trait
/// FileTyoe is the type of file that will be written to S3
pub struct S3Writer<H: ObjectStore> {
    filetype: FileType,
    fetcher: SchemaFetcher<H>,
}

impl<H: ObjectStore> S3Writer<H> {
    pub fn new(object_store: Arc<H>, filetype: FileType) -> Self {
        let fetcher = SchemaFetcher::new(object_store);
        Self { fetcher, filetype }
    }
}

#[async_trait::async_trait]
impl Writer for S3Writer<AmazonS3> {
    #[inline]
    async fn write(&self) -> anyhow::Result<()> {
        // get list of files from S3
        let files = self.fetcher.list_files().await?;
        let mut schemas = self
            .fetcher
            .fetch_file_from_s3(&self.filetype)
            .await?
            .pop()
            .unwrap();
        // iterate over files if file_name exists in schemas then skip
        // other wise fetch schema from ruspie and append it to schemas
        for file in files {
            let mut found = false;
            for schema in schemas.tables.iter() {
                if schema.name == file.name {
                    found = true;
                    break;
                }
            }
            if !found {
                let schema = self
                    .fetcher
                    .fetch_from_ruspie(&file.name, &file.extension)
                    .await?;
                if let Some(schema) = schema {
                    schemas.tables.push(schema);
                }
            }
        }

        // convert schemas to json
        // write to file
        let contents = serde_json::to_string(&vec![schemas])?;
        let mut file = tokio::fs::File::create("schema.json").await?;
        file.write_all(contents.as_bytes()).await?;

        // upload file to S3
        let path = self.filetype.s3_path();
        let mut data = Vec::new();
        let mut file = tokio::fs::File::open("schema.json").await?;
        file.read_to_end(&mut data).await?;
        let location: Path = path.path().try_into().unwrap();
        match self.fetcher.push_file_to_s3(location, data).await {
            Ok(_) => println!("Successfully wrote to S3"),
            Err(e) => println!("Error writing to S3: {:?}", e),
        };

        Ok(())
    }

    #[inline]
    fn schema_fetcher(&self) -> &SchemaFetcher<AmazonS3> {
        &self.fetcher
    }
}
