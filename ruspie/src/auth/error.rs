#![allow(dead_code)]

use axum::http;
use axum::http::Response;
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::{error::Error as StdError, fmt};

#[derive(Debug, thiserror::Error)]
pub enum AuthControllerError {
    #[error("API key `{0}` not found.")]
    ApiKeyNotFound(String),
    #[error("`uid` field value `{0}` is already an existing API key.")]
    ApiKeyAlreadyExists(String),
    #[error(transparent)]
    ApiKey(#[from] Error),
    #[error("Internal error: {0}")]
    Internal(Box<dyn StdError + Send + Sync + 'static>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`{0}` field is mandatory.")]
    MissingParameter(&'static str),
    #[error("`expiresAt` field value `{0}` is invalid. It should follow the RFC 3339 format to represents a date or datetime in the future or specified as a null value. e.g. 'YYYY-MM-DD' or 'YYYY-MM-DD HH:MM:SS'.")]
    InvalidApiKeyExpiresAt(Value),
    #[error("`description` field value `{0}` is invalid. It should be a string or specified as a null value.")]
    InvalidApiKeyDescription(Value),
    #[error(
        "`name` field value `{0}` is invalid. It should be a string or specified as a null value."
    )]
    InvalidApiKeyName(Value),
    #[error("`uid` field value `{0}` is invalid. It should be a valid UUID v4 string or omitted.")]
    InvalidApiKeyUid(Value),
    #[error("The `{0}` field cannot be modified for the given resource.")]
    ImmutableField(String),
}

#[derive(Serialize, thiserror::Error, Debug)]
pub struct AuthError {
    #[serde(serialize_with = "serialize_statuscode")]
    pub code: http::StatusCode,
    pub error: String,
    pub message: String,
}

fn serialize_statuscode<S>(x: &http::StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

impl From<http::Error> for AuthError {
    fn from(e: http::Error) -> Self {
        AuthError {
            error: "http_error".to_string(),
            message: e.to_string(),
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]({}): {}", self.code, self.error, self.message)
    }
}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::to_string(&self).unwrap();
        let body = axum::body::boxed(axum::body::Full::from(payload));

        Response::builder().status(self.code).body(body).unwrap()
    }
}

impl AuthError {
    pub fn new() -> Self {
        Self {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: "".to_string(),
            message: "".to_string(),
        }
    }
}
