use std::sync::Arc;

use object_store::{aws::AmazonS3, ObjectStore};

use crate::context::{Schema, Schemas};

use super::{Fetcher, FileType};

pub struct S3Fetcher {
    filetype: FileType,
    object_store: Arc<AmazonS3>,
}

impl S3Fetcher {
    pub fn new(object_store: Arc<AmazonS3>, filetpye: FileType) -> Self {
        Self {
            object_store,
            filetype: filetpye,
        }
    }
}

#[async_trait::async_trait]
impl Fetcher for S3Fetcher {
    async fn fetch(&self) -> anyhow::Result<Vec<Schema>> {
        let path = self.filetype.s3_path();
        let mut schemas = match self
            .object_store
            .get(&path.path().try_into().unwrap())
            .await
        {
            Ok(data) => {
                let data: Vec<u8> = data.bytes().await?.to_vec();
                let contents = String::from_utf8(data)?;
                serde_json::from_str(&contents).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    vec![Schemas { tables: vec![] }]
                })
            }
            Err(e) => {
                println!("File not found, creating new file, Error: {:?}", e);
                vec![Schemas { tables: vec![] }]
            }
        };
        Ok(schemas.pop().unwrap().tables)
    }
}
