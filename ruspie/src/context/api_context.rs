use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use columnq::{
    datafusion::{arrow, error::DataFusionError, prelude::DataFrame},
    encoding,
    error::{ColumnQError, QueryError},
    table::TableSource,
    ColumnQ,
};
use roapi::{context::RoapiContext, error::ApiErrResp};

#[derive(Debug)]
pub enum Source {
    FILESYSTEM,
    S3,
    INVALID,
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        if value.to_lowercase() == String::from("filesystem") {
            return Source::FILESYSTEM;
        } else if value.to_lowercase() == String::from("s3") {
            return Source::S3;
        }
        Source::INVALID
    }
}

#[async_trait]
pub trait RuspieApiContext: RoapiContext {
    async fn conf_table(&mut self, table_source: &TableSource) -> Result<(), ColumnQError>;
    async fn table_exists(&self, table_name: &str) -> bool;
    async fn query_rest_table_ruspie(
        &self,
        table_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError>;
    async fn query_graphql_ruspie(
        &self,
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError>;
    async fn query_sql_ruspie(
        &self,
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError>;
    fn get_source(&self) -> &Source;
}

pub struct RawRuspieApiContext {
    pub cq: ColumnQ,
    pub response_format: encoding::ContentType,
    pub source: Source,
}

impl RawRuspieApiContext {
    pub fn new() -> Self {
        let cq = ColumnQ::new();
        let source: Source = std::env::var("SOURCE")
            .unwrap_or_else(|_| "FILESYSTEM".to_string())
            .into();
        match source {
            Source::INVALID => panic!("unsupported source {:?}", &source),
            _ => {}
        }
        Self {
            cq,
            response_format: encoding::ContentType::default(),
            source,
        }
    }
}

#[async_trait]
impl RuspieApiContext for RawRuspieApiContext {
    async fn conf_table(&mut self, table_source: &TableSource) -> Result<(), ColumnQError> {
        self.cq.load_table(&table_source).await
    }

    async fn table_exists(&self, table_name: &str) -> bool {
        match self.cq.schema_map().get(table_name) {
            None => return false,
            _ => return true,
        };
    }
    async fn query_rest_table_ruspie(
        &self,
        table_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_rest_table(table_name, params).await
    }

    async fn query_graphql_ruspie(
        &self,
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_graphql(query).await
    }

    async fn query_sql_ruspie(
        &self,
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_sql(query).await
    }

    fn get_source(&self) -> &Source {
        &self.source
    }
}

#[async_trait]
impl RoapiContext for RawRuspieApiContext {
    #[inline]
    fn read_only_mode() -> bool {
        true
    }

    #[inline]
    async fn load_table(&self, _table: &TableSource) -> Result<(), ColumnQError> {
        Err(ColumnQError::Generic(
            "Table update not supported in read only mode".to_string(),
        ))
    }

    #[inline]
    async fn schemas_json_bytes(&self) -> Result<Vec<u8>, ApiErrResp> {
        serde_json::to_vec(self.cq.schema_map())
            .map_err(columnq::error::ColumnQError::from)
            .map_err(ApiErrResp::json_serialization)
    }

    #[inline]
    async fn table_schema_json_bytes(&self, table_name: &str) -> Result<Vec<u8>, ApiErrResp> {
        serde_json::to_vec(
            self.cq
                .schema_map()
                .get(table_name)
                .ok_or_else(|| ApiErrResp::not_found("invalid table name"))?
                .as_ref(),
        )
        .map_err(columnq::error::ColumnQError::from)
        .map_err(ApiErrResp::json_serialization)
    }

    #[inline]
    async fn query_graphql(
        &self,
        _query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        unreachable!()
    }

    #[inline]
    async fn query_sql(
        &self,
        _query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        unreachable!()
    }

    #[inline]
    async fn query_rest_table(
        &self,
        _table_name: &str,
        _params: &HashMap<String, String>,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        unreachable!()
    }

    #[inline]
    async fn kv_get(&self, _kv_name: &str, _key: &str) -> Result<Option<String>, QueryError> {
        unreachable!()
    }

    #[inline]
    async fn sql_to_df(&self, _query: &str) -> Result<Arc<DataFrame>, DataFusionError> {
        unreachable!()
    }

    #[inline]
    async fn get_response_format(&self) -> encoding::ContentType {
        self.response_format
    }
}
