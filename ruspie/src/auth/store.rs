#![allow(dead_code)]
use std::{sync::Arc, path::Path, fs::create_dir_all, cmp::Reverse};
use heed::{Env, Database, types::{ByteSlice, SerdeJson, DecodeIgnore}, EnvOpenOptions};
use hmac::{Hmac,Mac};
use sha2::Sha256;
use uuid::{Uuid, fmt::Hyphenated};

use super::{keys::Key, error::AuthControllerError};
type Result<T> = std::result::Result<T, AuthControllerError>;

const AUTH_STORE_SIZE: usize = 1_073_741_824; //1GiB
const AUTH_DB_PATH: &str = "auth";
const KEY_DB_NAME: &str = "api-keys";
const KEY_ID_ACTION_INDEX_EXPIRATION_DB_NAME: &str = "keyid-action-index-expiration";

#[derive(Clone)]
pub struct HeedAuthStore {
    pub env: Arc<Env>,
    pub keys: Database<ByteSlice, SerdeJson<Key>>,
    pub should_close_on_drop: bool
}

impl Drop for HeedAuthStore {
    fn drop(&mut self) {
        if self.should_close_on_drop && Arc::strong_count(&self.env) == 1 {
            self.env.as_ref().clone().prepare_for_closing();
        }
    }
}

pub fn open_auth_store_env(path: &Path) -> heed::Result<heed::Env> {
    let mut options = EnvOpenOptions::new();
    options.map_size(AUTH_STORE_SIZE); // 1GB
    options.max_dbs(2);
    options.open(path)
}

impl HeedAuthStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().join(AUTH_DB_PATH);
        create_dir_all(&path)?;
        let env = Arc::new(open_auth_store_env(path.as_ref())?);
        let keys = env.create_database(Some(KEY_DB_NAME))?;
        Ok(Self { env, keys, should_close_on_drop: true })
    }

    pub fn set_drop_on_close(&mut self, v: bool) {
        self.should_close_on_drop = v;
    }

    pub fn is_empty(&self) -> Result<bool> {
        let rtxn = self.env.read_txn()?;

        Ok(self.keys.len(&rtxn)? == 0)
    }

    pub fn put_api_key(&self, key: Key) -> Result<Key> {
        let uid = key.uid;
        let mut wtxn = self.env.write_txn()?;
        self.keys.put(&mut wtxn, uid.as_bytes(), &key)?;
        wtxn.commit()?;
        todo!()
    }

    pub fn get_api_key(&self, uid: Uuid) -> Result<Option<Key>> {
        let rtxn = self.env.read_txn()?;
        self.keys.get(&rtxn, uid.as_bytes()).map_err(|e| e.into())
    }

    pub fn get_uid_from_encoded_key(
        &self,
        encoded_key: &[u8],
        master_key: &[u8],
    ) -> Result<Option<Uuid>> {
        let rtxn = self.env.read_txn()?;
        let uid = self
            .keys
            .remap_data_type::<DecodeIgnore>()
            .iter(&rtxn)?
            .filter_map(|res| match res {
                Ok((uid, _)) => {
                    let (uid, _) = try_split_array_at(uid)?;
                    let uid = Uuid::from_bytes(*uid);
                    if generate_key_as_hexa(uid, master_key).as_bytes() == encoded_key {
                        Some(uid)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .next();

        Ok(uid)
    }

    pub fn delete_api_key(&self, uid: Uuid) -> Result<bool> {
        let mut wtxn = self.env.write_txn()?;
        let existing = self.keys.delete(&mut wtxn, uid.as_bytes())?;
        wtxn.commit()?;

        Ok(existing)
    }

    pub fn delete_all_keys(&self) -> Result<()> {
        let mut wtxn = self.env.write_txn()?;
        self.keys.clear(&mut wtxn)?;
        wtxn.commit()?;
        Ok(())
    }

    pub fn list_api_keys(&self) -> Result<Vec<Key>> {
        let mut list = Vec::new();
        let rtxn = self.env.read_txn()?;
        for result in self.keys.remap_key_type::<DecodeIgnore>().iter(&rtxn)? {
            let (_, content) = result?;
            list.push(content);
        }
        list.sort_unstable_by_key(|k| Reverse(k.created_at));
        Ok(list)
    }

}

pub fn try_split_at<T>(slice: &[T], mid: usize) -> Option<(&[T], &[T])> {
    if mid <= slice.len() {
        Some(slice.split_at(mid))
    } else {
        None
    }
}

pub fn try_split_array_at<T, const N: usize>(slice: &[T]) -> Option<(&[T; N], &[T])>
where
    [T; N]: for<'a> TryFrom<&'a [T]>,
{
    let (head, tail) = try_split_at(slice, N)?;
    let head = head.try_into().ok()?;
    Some((head, tail))
}

pub fn generate_key_as_hexa(uid: Uuid, master_key: &[u8]) -> String {
    // format uid as hyphenated allowing user to generate their own keys.
    let mut uid_buffer = [0; Hyphenated::LENGTH];
    let uid = uid.hyphenated().encode_lower(&mut uid_buffer);

    // new_from_slice function never fail.
    let mut mac = Hmac::<Sha256>::new_from_slice(master_key).unwrap();
    mac.update(uid.as_bytes());

    let result = mac.finalize();
    format!("{:x}", result.into_bytes())
}