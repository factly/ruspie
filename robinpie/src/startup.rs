use crate::context::S3FileContext;
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
        let bucket_name = std::env::var("S3_PATH").unwrap_or_else(|_| "ruspie".to_string());

        let s3 = AmazonS3Builder::from_env()
            .with_bucket_name(bucket_name)
            .with_allow_http(true)
            .build()
            .unwrap();
        Arc::new(s3)
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut ctx = S3FileContext::default(Self::build_object_store()).unwrap();
        let mut interval = tokio::time::interval(self.interval);
        println!("🚀 Robinpie started...",);
        loop {
            ctx.fetch_schemas_from_ruspie().await?;
            interval.tick().await;
        }
    }
}
