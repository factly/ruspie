#![allow(dead_code)]

use serde::{Serialize, Deserialize};
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

const PAGINATION_DEFAULT_LIMIT: fn() -> usize = || 20;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default)]
    pub offset: usize,
    #[serde(default = "PAGINATION_DEFAULT_LIMIT")]
    pub limit:usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationView<T> {
    pub results: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
}

impl Pagination {
    /// Given the full data to paginate, returns the selected section.
    pub fn auto_paginate_sized<T>(
        self,
        content: impl IntoIterator<Item = T> + ExactSizeIterator,
    ) -> PaginationView<T>
    where
        T: Serialize,
    {
        let total = content.len();
        let content: Vec<_> = content.into_iter().skip(self.offset).take(self.limit).collect();
        self.format_with(total, content)
    }

    /// Given an iterator and the total number of elements, returns the selected section.
    pub fn auto_paginate_unsized<T>(
        self,
        total: usize,
        content: impl IntoIterator<Item = T>,
    ) -> PaginationView<T>
    where
        T: Serialize,
    {
        let content: Vec<_> = content.into_iter().skip(self.offset).take(self.limit).collect();
        self.format_with(total, content)
    }

    /// Given the data already paginated + the total number of elements, it stores
    /// everything in a [PaginationResult].
    pub fn format_with<T>(self, total: usize, results: Vec<T>) -> PaginationView<T>
    where
        T: Serialize,
    {
        PaginationView { results, offset: self.offset, limit: self.limit, total }
    }
}

impl<T> PaginationView<T> {
    pub fn new(offset: usize, limit: usize, total: usize, results: Vec<T>) -> Self {
        Self { offset, limit, results, total }
    }
}