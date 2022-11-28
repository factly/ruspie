#![allow(dead_code, unused_variables)]
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{error::AuthError, parse_expiration_date, Result};

pub type KeyId = uuid::Uuid;

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
    pub fn create_from_value(value: Value) -> Result<Self> {
        let name = match value.get("name") {
            None | Some(Value::Null) => None,
            Some(des) => from_value::<String>(des.clone())
                .map(Some)
                .map_err(|_| AuthError::invalid_api_key_description(des.clone()))?,
        };

        let description = match value.get("description") {
            None | Some(Value::Null) => None,
            Some(des) => from_value::<String>(des.clone())
                .map(Some)
                .map_err(|_| AuthError::invalid_api_key_description(des.clone()))?,
        };

        let uid = value.get("uid").map_or_else(
            || Ok(Uuid::new_v4()),
            |uid| from_value(uid.clone()).map_err(|_| AuthError::invalid_api_key_uid(uid.clone())),
        )?;

        let expires_at = value
            .get("expiresAt")
            .map(parse_expiration_date)
            .ok_or(AuthError::missing_parameter("expiresAt"))??;

        let created_at = OffsetDateTime::now_utc();
        let updated_at = created_at;
        Ok(Self {
            name,
            uid,
            description,
            expires_at,
            created_at,
            updated_at,
        })
    }

    pub fn update_from_value(&mut self, value: Value) -> Result<()> {
        if let Some(des) = value.get("description") {
            let des = from_value(des.clone())
                .map_err(|_| AuthError::invalid_api_key_description(des.clone()));
            self.description = des?;
        }

        if let Some(des) = value.get("name") {
            let des =
                from_value(des.clone()).map_err(|_| AuthError::invalid_api_key_name(des.clone()));
            self.name = des?;
        }

        if value.get("uid").is_some() {
            return Err(AuthError::immutable_field("uid".to_string()));
        }

        if value.get("expiresAt").is_some() {
            return Err(AuthError::immutable_field("expiresAt".to_string()));
        }

        if value.get("createdAt").is_some() {
            return Err(AuthError::immutable_field("createdAt".to_string()));
        }

        if value.get("updatedAt").is_some() {
            return Err(AuthError::immutable_field("updatedAt".to_string()));
        }
        self.updated_at = OffsetDateTime::now_utc();

        Ok(())
    }

    pub fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let uid = Uuid::new_v4();
        Self {
            name: Some("Default Search API Key".to_string()),
            description: Some("Use it to search from the frontend".to_string()),
            uid,
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}
