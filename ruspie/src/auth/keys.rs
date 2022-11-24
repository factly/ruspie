use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

use super::error::AuthError;

pub type KeyId = uuid::Uuid;

type Result<T> = std::result::Result<T, AuthError>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub uid: KeyId,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    // pub fn create_from_value(value: Value) -> Result<Self> {
    //     let name = match value.get("name") {
    //         None | Some(Value::Null) => None,
    //         Some(des) => from_value(des.clone())
    //             .map(Some)
    //             .map_err(|_| E)?,
    //     };
    // }
}