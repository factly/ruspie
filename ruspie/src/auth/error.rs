#![allow(dead_code)]

use axum::http;
use axum::http::Response;
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::fmt::{self};

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

#[derive(Debug, thiserror::Error, Serialize)]
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
    #[error("Unknown error encountered")]
    UnknownError(String),
}

#[derive(Serialize, thiserror::Error, Debug)]
pub struct AuthError {
    #[serde(serialize_with = "serialize_statuscode")]
    pub code: http::StatusCode,
    pub error: AuthControllerError,
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
            error: AuthControllerError::Internal(e.to_string()),
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
    pub fn invalid_api_key_name(name: Value) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            error: AuthControllerError::ApiKey(Error::InvalidApiKeyName(name)),
            message: "API key name is invalid.".to_string(),
        }
    }

    pub fn invalid_api_key_description(des: Value) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            error: AuthControllerError::ApiKey(Error::InvalidApiKeyDescription(des)),
            message: "API key description is invalid".to_string(),
        }
    }

    pub fn invalid_api_key_uid(uid: Value) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            error: AuthControllerError::ApiKey(Error::InvalidApiKeyUid(uid)),
            message: "API key uid is invalid.".to_string(),
        }
    }

    pub fn missing_parameter(param: &'static str) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            error: AuthControllerError::ApiKey(Error::MissingParameter(param)),
            message: format!("Missing parameter {}", param),
        }
    }

    pub fn invalid_api_key_expires_at(value: Value) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            message: format!("Invalid API key expires at {}", value),
            error: AuthControllerError::ApiKey(Error::InvalidApiKeyExpiresAt(value)),
        }
    }

    pub fn immutable_field(value: String) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            message: format!("cannot change value {} of an immutable field", value),
            error: AuthControllerError::ApiKey(Error::ImmutableField(value)),
        }
    }

    pub fn create_dir_all_failed(err: &dyn std::error::Error) -> Self {
        AuthError {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: AuthControllerError::Internal(err.to_string()),
            message: err.to_string(),
        }
    }

    pub fn internal_error(err: String) -> Self {
        AuthError {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: AuthControllerError::Internal(err.clone()),
            message: err,
        }
    }

    pub fn api_key_already_exists(err: String) -> Self {
        AuthError {
            code: http::StatusCode::BAD_REQUEST,
            error: AuthControllerError::ApiKeyAlreadyExists(err.clone()),
            message: err,
        }
    }

    pub fn api_key_not_found(e: String) -> Self {
        AuthError {
            code: http::StatusCode::NOT_FOUND,
            error: AuthControllerError::ApiKeyNotFound(e.clone()),
            message: e,
        }
    }
}

impl From<heed::Error> for AuthError {
    fn from(e: heed::Error) -> Self {
        AuthError {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            error: AuthControllerError::Internal(e.to_string()),
            message: e.to_string(),
        }
    }
}
