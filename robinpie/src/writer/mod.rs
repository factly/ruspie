use fetcher::SchemaFetcher;
use object_store::aws::AmazonS3;
use serde::{Deserialize, Serialize};

pub mod fetcher;
pub mod mongo;
pub mod s3;

#[async_trait::async_trait]
/// Writer trait for writing schemas to a source
pub trait Writer {
    /// Writes a schema to the source
    async fn write(&self) -> anyhow::Result<()>;
    /// SchemaFetcher from which the schema will be fetched and list of files exist on S3 is
    /// fetched
    fn schema_fetcher(&self) -> &SchemaFetcher<AmazonS3>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFile {
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
pub struct ColumnItem {
    name: String,
    data_type: arrow::datatypes::DataType,
    nullable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaResponse {
    pub fields: Vec<ColumnItem>,
}

#[derive(Debug, Deserialize)]
pub struct SchemaErrorResponse {
    pub error: String,
    pub code: u16,
    pub message: String,
}

impl From<SchemaResponse> for Schema {
    fn from(schema_resp: SchemaResponse) -> Self {
        Schema {
            columns: schema_resp.fields,
        }
    }
}
