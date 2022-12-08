#![allow(dead_code)]


use axum::{response::IntoResponse, http::{Response, StatusCode}};
use roapi::error::ApiErrResp;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("`{0}` field is mandatory.")]
    MissingParameter(&'static str),
    #[error("`actions` field value `{0}` is invalid. It should be an array of string representing action names.")]
    InvalidApiKeyActions(Value),
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

#[derive(Debug, thiserror::Error, Serialize)]
pub enum AuthControllerError {
    #[error("API key `{0}` not found.")]
    ApiKeyNotFound(String),
    #[error("`uid` field value `{0}` is already an existing API key.")]
    ApiKeyAlreadyExists(String),
    #[error(transparent)]
    ApiKey(#[from] Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AuthControllerError {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::to_string(&self).unwrap();
        let body = axum::body::boxed(axum::body::Full::from(payload));

        let code =  match self {
            Self::ApiKey(_) => StatusCode::BAD_REQUEST,
            Self::ApiKeyAlreadyExists(_) => StatusCode::CONFLICT,
            Self::ApiKeyNotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        };
        Response::builder().status(code).body(body).unwrap()
    }
}

impl From<heed::Error> for AuthControllerError {
    fn from(e: heed::Error) -> Self {
        AuthControllerError::Internal(e.to_string())
    }
}

impl From<std::io::Error> for AuthControllerError {
    fn from(e: std::io::Error) -> Self {
        AuthControllerError::Internal(e.to_string())
    }
}

impl From<ApiErrResp> for AuthControllerError {
    fn from(_: ApiErrResp) -> Self {
        todo!()
    }
}
