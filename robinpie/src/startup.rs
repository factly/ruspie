use crate::context::{S3FileContext, SchemaFileType};
use object_store::aws::{AmazonS3, AmazonS3Builder};
use std::sync::Arc;

pub struct Application {
    interval: std::time::Duration,
}

impl Application {
    pub fn new(interval: std::time::Duration) -> Self {
        Self { interval }
    }
    fn build_object_store() -> Arc<AmazonS3> {
        let bucket_name = std::env::var("S3_PATH").unwrap();

        let s3 = AmazonS3Builder::from_env()
            .with_bucket_name(bucket_name)
            .with_allow_http(true)
            .build()
            .unwrap();
        Arc::new(s3)
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let ctx = S3FileContext {
            schema_file_type: SchemaFileType::Json,
            schema_file_name: "schemas".to_string(),
            object_store: Self::build_object_store(),
        };
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
        }
    }
}
