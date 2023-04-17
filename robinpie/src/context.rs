#![allow(dead_code)]

use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub enum SchemaFileType {
    Json,
    Parquet,
}

impl SchemaFileType {
    pub fn s3_path(&self, name: String) -> url::Url {
        let extension = match self {
            SchemaFileType::Json => "json",
            SchemaFileType::Parquet => "parquet",
        };

        let path = std::env::var("S3_PATH").unwrap();
        let path: String = format!("s3://{}/{}.{}", path, name, extension);
        url::Url::parse(&path).unwrap()
    }
}

pub struct S3FileContext<H: object_store::ObjectStore> {
    pub schema_file_type: SchemaFileType,
    pub schema_file_name: String,
    pub object_store: Arc<H>,
    pub schemas_to_fetched: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaFile {
    tables: Vec<TableItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableItem {
    name: String,
    extension: String,
    #[serde(default = "default_schema")]
    schema: Schema,
}

impl Default for TableItem {
    fn default() -> Self {
        TableItem {
            name: "".to_string(),
            extension: "".to_string(),
            schema: default_schema(),
        }
    }
}

fn default_schema() -> Schema {
    Schema { columns: vec![] }
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    columns: Vec<ColumnItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnItem {
    name: String,
    data_type: String,
    nullable: bool,
}

impl<H: object_store::ObjectStore> S3FileContext<H> {
    async fn list_files(&self) -> anyhow::Result<Vec<TableItem>> {
        let mut list_stream = self.object_store.list(None).await.unwrap();
        let mut files: Vec<TableItem> = vec![];

        while let Some(file) = list_stream.next().await {
            let file = file.unwrap();
            if file.location.extension() == Some("parquet")
                || file.location.extension() == Some("csv")
            {
                let mut table_item = TableItem::default();
                table_item.extension = file.location.extension().unwrap().to_string();
                table_item.name = file.location.filename().unwrap().to_string();
                files.push(table_item);
            }
        }

        Ok(files)
    }

    async fn fetch_schema(&self) -> anyhow::Result<()> {
        let path = self.schema_file_type.s3_path(self.schema_file_name.clone());
        let _ = match self
            .object_store
            .get(&path.path().try_into().unwrap())
            .await
        {
            Ok(data) => {
                let data = data.bytes().await.unwrap();
                let data = data.to_vec();
                let contents = String::from_utf8(data).unwrap();
                println!("{}", contents)
            }
            Err(e) => println!("cannot read {:?}", e),
        };
        Ok(())
    }

    pub async fn fetch_schemas_from_ruspie(&self) -> anyhow::Result<()> {
        todo!()
    }
}
