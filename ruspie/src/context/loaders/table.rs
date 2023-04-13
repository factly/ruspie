use std::{sync::Arc, time::Duration};

use columnq::table::{TableLoadOption, TableSource};
use log::{error, info};
use tokio::sync::Mutex;

use crate::context::{api_context::RuspieApiContext, Schema, Schemas};

use super::schema::S3FileSchemaLoader;

pub struct TableReloader<H: RuspieApiContext> {
    pub schemas: Schemas,
    pub ctx: Arc<Mutex<H>>,
    pub interval: Duration,
    pub loader: S3FileSchemaLoader,
}

impl<H: RuspieApiContext> TableReloader<H> {
    pub async fn run(mut self) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(self.interval);

        loop {
            self.schemas = self.loader.load().await?.pop().unwrap();

            interval.tick().await;

            for Schema {
                name,
                extension,
                schema,
            } in self.schemas.tables.iter()
            {
                let map = create_serde_map!(extension, use_memory_table);
                let opt: TableLoadOption =
                    serde_json::from_value(serde_json::Value::Object(map)).unwrap();

                let path = std::env::var("S3_PATH").unwrap_or_else(|_| String::from("ruspie/"));
                let path = format!("s3://{}/{}.{}", path, name, extension);

                let source = TableSource::new(name, path)
                    .with_option(opt)
                    .with_schema(schema.clone());

                let mut ctx = self.ctx.lock().await;
                match ctx.conf_table(&source).await {
                    Ok(_) => info!("🚀 TableReloader reloaded schema of table {}", name),
                    Err(e) => error!("failed to reload schema for {:?}", e),
                }
            }
        }
    }
}
