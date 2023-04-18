#![allow(dead_code)]

use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

        let path = std::env::var("S3_PATH").unwrap_or_else(|_| "ruspie".to_string());
        let path: String = format!("s3://{}/{}.{}", path, name, extension);
        url::Url::parse(&path).unwrap()
    }
}

pub struct S3FileContext<H: object_store::ObjectStore> {
    pub schema_file_type: SchemaFileType,
    pub schema_file_name: String,
    pub object_store: Arc<H>,
    schemas: Vec<SchemaFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchemaFile {
    tables: Vec<TableItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableItem {
    name: String,
    extension: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Schema {
    columns: Vec<ColumnItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnItem {
    name: String,
    data_type: String,
    nullable: Option<bool>,
}

impl<H: object_store::ObjectStore> S3FileContext<H> {
    pub fn default(object_store: Arc<H>) -> anyhow::Result<S3FileContext<H>> {
        let schema_file_type = SchemaFileType::Json;
        let schema_file_name = "schemas".to_string();
        let schemas = vec![SchemaFile { tables: vec![] }];

        Ok(S3FileContext {
            schema_file_type,
            schema_file_name,
            object_store,
            schemas,
        })
    }

    async fn list_files(&self) -> anyhow::Result<Vec<TableItem>> {
        let mut list_stream = self.object_store.list(None).await.unwrap();
        let mut files: Vec<TableItem> = vec![];

        while let Some(file) = list_stream.next().await {
            let file = file.map_err(|e| println!("{:?}", e)).unwrap();
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

    async fn fetch_schema(&mut self) -> anyhow::Result<()> {
        let path = self.schema_file_type.s3_path(self.schema_file_name.clone());
        let schemas = match self
            .object_store
            .get(&path.path().try_into().unwrap())
            .await
        {
            Ok(data) => {
                let data: Vec<u8> = data.bytes().await.unwrap().to_vec();
                let contents: String = String::from_utf8(data).unwrap();
                serde_json::from_str(&contents).unwrap()
            }
            Err(e) => {
                println!("cannot read {:?}", e);
                vec![]
            }
        };
        self.schemas = schemas;
        Ok(())
    }

    async fn schemas_to_fetch(&mut self) -> Vec<TableItem> {
        let files_in_s3 = self.list_files().await.unwrap();
        let _ = self.fetch_schema().await.unwrap();
        let mut schemas_to_fetched: Vec<TableItem> = vec![];

        for file in files_in_s3 {
            let mut found = false;
            for schema in self.schemas[0].tables.iter() {
                if file.name == schema.name && file.extension == schema.extension {
                    found = true;
                    break;
                }
            }
            if !found {
                let table_item = TableItem {
                    name: file.name.clone(),
                    extension: file.extension.clone(),
                    ..Default::default()
                };
                schemas_to_fetched.push(table_item);
            }
        }

        schemas_to_fetched
    }

    pub async fn fetch_schemas_from_ruspie(&mut self) -> anyhow::Result<()> {
        let schemas_to_fetch = self.schemas_to_fetch().await;
        println!(
            "before fetching ==============>{:?}",
            self.schemas[0].tables.len()
        );
        for schema in schemas_to_fetch {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());
            headers.insert("Accept", "application/json".parse().unwrap());
            headers.insert("FILE-EXTENSION", schema.extension.parse().unwrap());

            let client = reqwest::Client::new();
            let url =
                std::env::var("RUSPIE_URL").unwrap_or_else(|_| "http://0.0.0.0:8080".to_string());
            let url = format!("{}/api/schema/{}", url, schema.name);

            let response = client.get(&url).headers(headers).send().await;
            match response {
                Ok(d) => {
                    let schema_resp: SchemaResponse = d.json().await.unwrap();
                    self.schemas[0].tables.push(TableItem {
                        name: schema.name,
                        extension: schema.extension,
                        schema: Schema {
                            columns: schema_resp.fields,
                        },
                    })
                }
                Err(e) => println!("error: {:?}", e),
            }
        }

        println!(
            "after fetching ==============>{:?}",
            self.schemas[0].tables.len()
        );
        self.write_to_json().await.unwrap();
        let file = tokio::fs::File::open("schemas.json").await.unwrap();
        match self.put_object_to_s3(&self.schema_file_name, file).await {
            Ok(d) => println!("done, {:?}", d),
            Err(e) => println!("error: {:?}", e),
        }

        Ok(())
    }
    async fn write_to_json(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string(&self.schemas).unwrap();
        let mut file = tokio::fs::File::create("schemas.json").await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    async fn put_object_to_s3(
        &self,
        path: &str,
        mut content: tokio::fs::File,
    ) -> anyhow::Result<()> {
        let path = self.schema_file_type.s3_path(path.to_string());
        let mut data = Vec::new();
        content.read_to_end(&mut data).await?;
        let location = path.path().try_into().unwrap();
        self.object_store.put(&location, data.into()).await?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct SchemaResponse {
    fields: Vec<ColumnItem>,
}
