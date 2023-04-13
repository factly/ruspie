use columnq::table::TableSchema;

pub mod api_context;
pub mod auth;
pub mod loaders;

#[derive(serde::Deserialize, Clone)]
pub struct Schemas {
    pub tables: Vec<Schema>,
}

#[derive(serde::Deserialize, Clone)]
pub struct Schema {
    name: String,
    extension: String,
    schema: TableSchema,
}
