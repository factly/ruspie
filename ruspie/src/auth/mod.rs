#![allow(dead_code)]
use std::{sync::Arc, path::Path};

use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use self::{store::{HeedAuthStore, generate_key_as_hexa}, error::AuthControllerError, keys::Key};

mod store;
mod error;
pub mod keys;

type Result<T> = std::result::Result<T, AuthControllerError>;

#[derive(Clone)]
pub struct AuthController {
    store: Arc<HeedAuthStore>,
    master_key: Option<String>,
}

impl AuthController {
    pub fn new(db_path: impl AsRef<Path>, master_key: &Option<String>) -> Result<Self> {
        let store = HeedAuthStore::new(db_path)?;

        Ok(Self { store: Arc::new(store), master_key: master_key.clone() })
    }

    pub fn create_key(&self, value: Value) -> Result<Key> {
        let key = Key::create_from_value(value)?;
        match self.store.get_api_key(key.uid)? {
            Some(_) => Err(AuthControllerError::ApiKeyAlreadyExists(key.uid.to_string())),
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
            .ok_or_else(|| AuthControllerError::ApiKeyNotFound(uid.to_string()))
    }

    pub fn get_optional_uid_from_encoded_key(&self, encoded_key: &[u8]) -> Result<Option<Uuid>> {
        match &self.master_key {
            Some(master_key) => {
                self.store.get_uid_from_encoded_key(encoded_key, master_key.as_bytes())
            }
            None => Ok(None),
        }
    }

    pub fn get_uid_from_encoded_key(&self, encoded_key: &str) -> Result<Uuid> {
        self.get_optional_uid_from_encoded_key(encoded_key.as_bytes())?
            .ok_or_else(|| AuthControllerError::ApiKeyNotFound(encoded_key.to_string()))
    }

    pub fn list_keys(&self) -> Result<Vec<Key>> {
        self.store.list_api_keys()
    }

    pub fn delete_key(&self, uid: Uuid) -> Result<()> {
        if self.store.delete_api_key(uid)? {
            Ok(())
        } else {
            Err(AuthControllerError::ApiKeyNotFound(uid.to_string()))
        }
    }

    pub fn get_master_key(&self) -> Option<&String> {
        self.master_key.as_ref()
    }

    /// Generate a valid key from a key id using the current master key.
    /// Returns None if no master key has been set.
    pub fn generate_key(&self, uid: Uuid) -> Option<String> {
        self.master_key.as_ref().map(|master_key| generate_key_as_hexa(uid, master_key.as_bytes()))
    }

    pub fn is_key_authorized(
        &self,
        uid: Uuid,
    ) -> Result<bool> {
        let key = self.get_key(uid)?;
        match key.expires_at {
            None => Ok(false),
            Some(exp) => Ok(OffsetDateTime::now_utc() < exp)
        }
    }

    pub fn raw_delete_all_keys(&mut self) -> Result<()> {
        self.store.delete_all_keys()
    }

    /// Delete all the keys in the DB.
    pub fn raw_insert_key(&mut self, key: Key) -> Result<()> {
        self.store.put_api_key(key)?;
        Ok(())
    }
}