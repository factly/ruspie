use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use columnq::{
    datafusion::{arrow, error::DataFusionError, prelude::DataFrame},
    error::{ColumnQError, QueryError},
    table::TableSource,
    ColumnQ,
};
use roapi::{context::RoapiContext, error::ApiErrResp};

#[async_trait]
pub trait FactlyApiContext: RoapiContext {
    async fn conf_table(&mut self, table_source: &TableSource) -> Result<(), ColumnQError>;
    async fn get_table(&self,table_name: &str) -> bool;
    async fn query_rest_table_2(
        &self,
        table_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<Vec<arrow::record_batch::RecordBatch>>, QueryError>;
}

pub struct RawFactlyApiContext {
    pub cq: ColumnQ,
}

impl RawFactlyApiContext {
    pub fn new() -> Self {
        let cq = ColumnQ::new();
        Self { cq }
    }
}

#[async_trait]
impl FactlyApiContext for RawFactlyApiContext {
    async fn conf_table(&mut self, table_source: &TableSource) -> Result<(), ColumnQError> {
        self.cq.load_table(&table_source).await
    }

    async fn get_table(&self, table_name: &str) -> bool {
        match self.cq.schema_map().get(table_name) {
            Some (_) => return true,
            None => return false
        };
        
    }
    async fn query_rest_table_2(
        &self,
        table_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<Vec<arrow::record_batch::RecordBatch>>, QueryError> {
        self.cq.query_rest_table_2(table_name, params).await
    }
}

#[async_trait]
impl RoapiContext for RawFactlyApiContext {
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
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_graphql(query).await
    }

    #[inline]
    async fn query_sql(
        &self,
        query: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_sql(query).await
    }

    #[inline]
    async fn query_rest_table(
        &self,
        table_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, QueryError> {
        self.cq.query_rest_table(table_name, params).await
    }

    #[inline]
    async fn kv_get(&self, kv_name: &str, key: &str) -> Result<Option<String>, QueryError> {
        Ok(self.cq.kv_get(kv_name, key)?.cloned())
    }

    #[inline]
    async fn sql_to_df(&self, query: &str) -> Result<Arc<DataFrame>, DataFusionError> {
        self.cq.dfctx.sql(query).await
    }
}