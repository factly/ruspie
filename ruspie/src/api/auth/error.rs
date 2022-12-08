use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use roapi::error::ApiErrResp;
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum AuthenticationError {
    #[error("The Authorization header is missing. It must use the bearer authorization method.")]
    MissingAuthorizationHeader,
    #[error("The provided API key is invalid.")]
    InvalidToken,
    // Triggered on configuration error.
    #[error("An internal error has occurred. `Irretrievable state`.")]
    IrretrievableState,
    #[error("Meilisearch is running without a master key. To access this API endpoint, you must have set a master key at launch.")]
    MissingMasterKey,
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::to_string(&self).unwrap();
        let body = axum::body::boxed(axum::body::Full::from(payload));

        let code = match self {
            Self::MissingAuthorizationHeader => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::CONFLICT,
            Self::IrretrievableState => StatusCode::NOT_FOUND,
            Self::MissingMasterKey => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder().status(code).body(body).unwrap()
    }
}

impl AuthenticationError {
    pub fn from_msg(message: String, code: StatusCode, error: String) -> ApiErrResp {
        ApiErrResp {
            code,
            error,
            message,
        }
    }
}

impl From<ApiErrResp> for AuthenticationError {
    fn from(e: ApiErrResp) -> Self {
        match e.message.as_str() {
            "Invalid Token" => AuthenticationError::InvalidToken,
            "Missing Auth Header" => AuthenticationError::MissingAuthorizationHeader,
            "Missing Master Key" => AuthenticationError::MissingMasterKey,
            _ => AuthenticationError::IrretrievableState
        }
    }
}