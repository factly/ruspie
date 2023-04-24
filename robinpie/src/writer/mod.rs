use fetcher::SchemaFetcher;
use object_store::aws::AmazonS3;
use serde::{Deserialize, Serialize};

use crate::context::Source;

pub mod fetcher;
pub mod mongo;

#[async_trait::async_trait]
/// Writer trait for writing schemas to a source
pub trait Writer {
    /// Writes a schema to the source
    async fn write(&self) -> anyhow::Result<()>;
    /// Source to which the schema will be written
    fn source(&self) -> &Source;
    /// SchemaFetcher from which the schema will be fetched and list of files exist on S3 is
    /// fetched
    fn schema_fetch(&self) -> &SchemaFetcher<AmazonS3>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchemaFile {
    tables: Vec<TableItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableItem {
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
    data_type: arrow::datatypes::DataType,
    nullable: Option<bool>,
}
