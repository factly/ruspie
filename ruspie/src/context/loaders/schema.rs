#![allow(dead_code)]
use columnq::table::{TableLoadOption, TableSource};

use crate::context::{
    api_context::{RawRuspieApiContext, RuspieApiContext},
    Schemas,
};

pub enum SchemaFileType {
    Json,
    Parquet,
}

impl SchemaFileType {
    pub fn file_source(&self, name: String) -> TableSource {
        let extension = match self {
            SchemaFileType::Json => "json".to_string(),
            SchemaFileType::Parquet => "parquet".to_string(),
        };

        let map = create_serde_map!(extension);
        let opt: TableLoadOption = serde_json::from_value(serde_json::Value::Object(map)).unwrap();

        let path = std::env::var("S3_PATH").unwrap_or_else(|_| String::from("ruspie/"));
        let path = format!("s3://{}/{}.{}", path, name, extension);

        TableSource::new(name, path).with_option(opt)
    }
}

pub struct S3FileSchemaLoader {
    filename: String,
    file_type: SchemaFileType,
}

impl S3FileSchemaLoader {
    pub fn new(filename: String, file_type: SchemaFileType) -> Self {
        Self {
            filename,
            file_type,
        }
    }
    pub async fn load(&self) -> anyhow::Result<Vec<Schemas>> {
        let mut ctx = RawRuspieApiContext::new();
        let _ = ctx
            .conf_table(&self.file_type.file_source(self.filename.clone()))
            .await
            .unwrap();
        let schemas = ctx.query_sql_ruspie("SELECT * FROM schemas").await.unwrap();
        let schemas = columnq::encoding::json::record_batches_to_bytes(&schemas).unwrap();

        let schemas: Vec<Schemas> = serde_json::from_slice(&schemas).unwrap();
        Ok(schemas)
    }
}
