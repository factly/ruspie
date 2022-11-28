#![allow(dead_code, unused_variables)]

use std::path::Path;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use super::error::AuthError;
use super::keys::Key;
use super::store::HeedAuthStore;
use super::{generate_key_as_hexa, Result};

#[derive(Clone)]
pub struct AuthController {
    store: Arc<HeedAuthStore>,
    master_key: Option<String>,
}

impl AuthController {
    pub fn new(db_path: impl AsRef<Path>, master_key: &Option<String>) -> Result<Self> {
        let store = HeedAuthStore::new(db_path)?;

        if store.is_empty()? {
            store.put_api_key(super::keys::Key::default())?;
        }

        Ok(Self {
            store: Arc::new(store),
            master_key: master_key.clone(),
        })
    }

    pub fn create_key(&self, value: Value) -> Result<Key> {
        let key = Key::create_from_value(value)?;
        match self.store.get_api_key(key.uid)? {
            Some(_) => Err(AuthError::api_key_already_exists(format!(
                "API key {} already exists",
                key.uid.to_string()
            ))),
            None => self.store.put_api_key(key),
        }
    }
    pub fn update_key(&self, uid: Uuid, value: Value) -> Result<Key> {
        let mut key = self.get_key(uid)?;
        key.update_from_value(value)?;
        self.store.put_api_key(key)
    }
    pub fn get_key(&self, uid: Uuid) -> Result<Key> {
        self.store
            .get_api_key(uid)?
            .ok_or_else(|| AuthError::api_key_not_found(format!("ApiKey {} not found", uid)))
    }

    pub fn get_optional_uid_from_encoded_key(&self, encoded_key: &[u8]) -> Result<Option<Uuid>> {
        match &self.master_key {
            Some(master_key) => self
                .store
                .get_uid_from_encoded_key(encoded_key, master_key.as_bytes()),
            None => Ok(None),
        }
    }

    pub fn get_uid_from_encoded_key(&self, encoded_key: &str) -> Result<Uuid> {
        self.get_optional_uid_from_encoded_key(encoded_key.as_bytes())?
            .ok_or_else(|| AuthError::api_key_not_found(encoded_key.to_string()))
    }

    pub fn list_keys(&self) -> Result<Vec<Key>> {
        self.store.list_api_keys()
    }

    pub fn delete_key(&self, uid: Uuid) -> Result<()> {
        if self.store.delete_api_key(uid)? {
            Ok(())
        } else {
            Err(AuthError::api_key_not_found(uid.to_string()))
        }
    }

    pub fn get_master_key(&self) -> Option<&String> {
        self.master_key.as_ref()
    }
    /// Generate a valid key from a key id using the current master key.
    /// Returns None if no master key has been set.
    pub fn generate_key(&self, uid: Uuid) -> Option<String> {
        self.master_key
            .as_ref()
            .map(|master_key| generate_key_as_hexa(uid, master_key.as_bytes()))
    }

    /// Delete all the keys in the DB.
    pub fn raw_delete_all_keys(&mut self) -> Result<()> {
        self.store.delete_all_keys()
    }

    /// Delete all the keys in the DB.
    pub fn raw_insert_key(&mut self, key: Key) -> Result<()> {
        self.store.put_api_key(key)?;
        Ok(())
    }
}
