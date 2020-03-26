use crate::error::DaemonError;
use crate::version_control::dataset::VCDataset;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Config {
    pub cache: Db,
    pub state: MemoryCache,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryCache {
    pub dbs: RwLock<HashMap<String, RwLock<VCDataset>>>,
}

impl From<&MemoryCache> for sled::IVec {
    fn from(cache: &MemoryCache) -> sled::IVec {
        let string = serde_json::to_string(&cache).expect("Serializing failed");
        string.into_bytes().into()
    }
}

impl From<sled::IVec> for MemoryCache {
    fn from(ivec: sled::IVec) -> MemoryCache {
        let string = String::from_utf8(ivec.to_vec()).expect("Converting bytes to string failed.");
        serde_json::from_str(&string).expect("Deserializing cache failed")
    }
}

impl MemoryCache {
    fn add_something(&self) -> Result<(), DaemonError> {
        Ok(())
    }
}
