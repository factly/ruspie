use columnq::arrow::datatypes::DataType;
use columnq::table::{TableColumn, TableSchema};
use serde::Deserialize;
pub mod api_context;
pub mod auth;
pub mod loaders;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Schemas {
    pub tables: Vec<Schema>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Schema {
    name: String,
    extension: String,
    schema: TableSchemaFetch,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct TableSchemaFetch {
    columns: Vec<ColumnItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnItem {
    name: String,
    data_type: DataType,
    nullable: Option<bool>,
}

impl From<TableSchemaFetch> for TableSchema {
    fn from(value: TableSchemaFetch) -> Self {
        let columns: Vec<TableColumn> = value
            .columns
            .iter()
            .map(|c| TableColumn {
                name: c.name.clone(),
                data_type: c.data_type.clone(),
                nullable: c.nullable.unwrap_or(false),
            })
            .collect();
        Self { columns }
    }
}
