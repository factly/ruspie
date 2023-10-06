use axum::{response::IntoResponse, Extension, Json};
use serde_derive::{Deserialize, Serialize};

use crate::context::openai::{OpenAIAPIRequestError, OpenAIContext};

#[derive(Deserialize, Serialize, Debug)]
pub struct TextToSQLBody {
    query: String,
    tablename: String,
    schema: Option<String>,
}

pub async fn text_to_sql(
    Extension(openai_ctx): Extension<OpenAIContext>,
    Json(body): Json<TextToSQLBody>,
) -> Result<impl IntoResponse, OpenAIAPIRequestError> {
    let payload = format!(
        r#"
     TEXT TO SQL CONVERSION
           TABLE_NAME: {}\n\
           SCHEMA: {:?}\
           NOTE: RETURN ONLY SQL, ALL THE KEYS IN THE JSON_RESPONSE ARE COLUMN NAMES , ALWAYS USE DOUBLE QUOTES FOR TABLE NAMES AND COLUMN NAMES IN SQL AND ALWAYS USE SINGLE QUOTES FOR VALUES IN SQL
           QUERY: ${}
     	  EXAMPLE: SELECT * FROM "table_name" WHERE "column_name" = 'value'
    
        "#,
        body.tablename, body.schema, body.query
    );
    let sql = openai_ctx.generate(payload.into()).await?;
    return Ok(Json(sql));
}
