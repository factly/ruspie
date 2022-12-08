#![allow(dead_code)]

use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::context::auth::{
    context::AuthContext,
    keys::Key,
};

pub mod handlers;
pub mod middleware;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KeyView {
    name: Option<String>,
    description: Option<String>,
    key: String,
    uid: Uuid,
    #[serde(serialize_with = "time::serde::rfc3339::option::serialize")]
    expires_at: Option<OffsetDateTime>,
    #[serde(serialize_with = "time::serde::rfc3339::serialize")]
    created_at: OffsetDateTime,
    #[serde(serialize_with = "time::serde::rfc3339::serialize")]
    updated_at: OffsetDateTime,
}

impl KeyView {
    fn from_key<A>(key: Key, auth: &A) -> Self 
    where A: AuthContext 
    {
        let generated_key = auth.generate_key(key.uid).unwrap_or_default();

        KeyView {
            name: key.name,
            description: key.description,
            key: generated_key,
            uid: key.uid,
            expires_at: key.expires_at,
            created_at: key.created_at,
            updated_at: key.updated_at,
        }
    }
}
