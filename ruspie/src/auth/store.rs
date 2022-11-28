#![allow(dead_code)]

use std::{fs, path::Path, sync::Arc, cmp::Reverse};

use super::{error::AuthError, keys::Key, Result, generate_key_as_hexa};
use heed::{
    types::{ByteSlice, SerdeJson, DecodeIgnore},
    Database, Env, EnvOpenOptions,
};
use uuid::Uuid;

const AUTH_STORE_SIZE: usize = 1_073_741_824; //1GiB
const AUTH_DB_PATH: &str = "auth";
const KEY_DB_NAME: &str = "api-keys";
const KEY_ID_ACTION_INDEX_EXPIRATION_DB_NAME: &str = "keyid-action-index-expiration";

#[derive(Clone)]
pub struct HeedAuthStore {
    env: Arc<Env>,
    keys: Database<ByteSlice, SerdeJson<Key>>,
    should_close_on_drop: bool,
}

impl Drop for HeedAuthStore {
    fn drop(&mut self) {
        if self.should_close_on_drop && Arc::strong_count(&self.env) == 1 {
            self.env.as_ref().clone().prepare_for_closing();
        }
    }
}

pub fn open_auth_store_env(path: &Path) -> heed::Result<Env> {
    let mut options = EnvOpenOptions::new();
    options.map_size(AUTH_STORE_SIZE); // 1GB
    options.max_dbs(2);
    options.open(path)
}

impl HeedAuthStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().join(AUTH_DB_PATH);
        fs::create_dir_all(&path).map_err(|e| AuthError::create_dir_all_failed(&e))?;
        let env = Arc::new(
            open_auth_store_env(path.as_ref())
                .map_err(|e| AuthError::internal_error(e.to_string()))?,
        );
        let keys = env
            .create_database(Some(KEY_DB_NAME))?;

        Ok(Self {
            env,
            keys,
            should_close_on_drop: true,
        })
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

        Ok(key)
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
                    let (uid, _) = super::try_split_array_at(uid)?;
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

    // fn delete_key_from_inverted_db(&self, wtxn: &mut RwTxn, key: &KeyId) -> Result<()> {
    //     let mut iter = self
    //         .action_keyid_index_expiration
    //         .remap_types::<ByteSlice, DecodeIgnore>()
    //         .prefix_iter_mut(wtxn, key.as_bytes())?;
    //     while iter.next().transpose()?.is_some() {
    //         // safety: we don't keep references from inside the LMDB database.
    //         unsafe { iter.del_current()? };
    //     }

    //     Ok(())
    // }
}
