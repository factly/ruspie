use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde_json::Value;

use crate::context::auth::{
    context::{AuthContext, RawAuthContext},
    error::AuthControllerError,
};

use super::{KeyView, Pagination};

pub async fn create_api_key(
    Json(payload): Json<Value>,
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    let key = auth_controller.create_key(payload)?;
    Ok(Json(KeyView::from_key(key, &auth_controller)))
}

pub async fn get_api_keys(
    paginate: axum::extract::Query<Pagination>,
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    let page_view = tokio::task::spawn_blocking(move || -> Result<_, AuthControllerError> {
        let keys = auth_controller.list_keys()?;
        let page_view = paginate.auto_paginate_sized(
            keys.into_iter()
                .map(|k| KeyView::from_key(k, &auth_controller)),
        );

        Ok(page_view)
    })
    .await
    .map_err(|e| AuthControllerError::Internal(e.to_string()))??;
    Ok(Json(page_view))
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

pub async fn invalidate_key(
    Path(key_id): Path<uuid::Uuid>,
    Extension(auth_controller): Extension<RawAuthContext>,
) -> Result<impl IntoResponse, AuthControllerError> {
    auth_controller
        .invalidate_key(key_id)
        .map(|key| Json(KeyView::from_key(key, &auth_controller)))
}
