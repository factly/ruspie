use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde_json::Value;

use crate::context::auth::{
    context::{AuthContext, RawAuthContext},
    error::AuthControllerError,
};

use super::KeyView;

pub async fn create_api_key(
    Json(payload): Json<Value>,
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    let key = auth_controller.create_key(payload)?;
    Ok(Json(KeyView::from_key(key, &auth_controller)))
}

pub async fn get_api_keys(
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    let keys = auth_controller.list_keys()?;
    let keys = keys
        .iter()
        .map(move |key| KeyView::from_key(key.clone(), &auth_controller))
        .collect::<Vec<KeyView>>();

    Ok(Json(keys))
}

pub async fn delete_api_key(
    Path(key_id): Path<uuid::Uuid>,
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    auth_controller.delete_key(key_id)
}

pub async fn update_api_key(
    Path(key_id): Path<uuid::Uuid>,
    Extension(auth_controller): Extension<RawAuthContext>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, AuthControllerError> {
    auth_controller
        .update_key(key_id, payload)
        .map(|key| Json(KeyView::from_key(key, &auth_controller)))
}
