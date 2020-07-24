//! Module which contains storage which is shared by the different endpoint handlers. It has a reference to the `local_config` which is a key-value store used to store DatasetConfigs, but also
//! a HashMap where the available data sets are stored in memory, for quicker access.
use iterum_rust::vc::Dataset;
use sled::Db;
use std::collections::HashMap;
use std::sync::RwLock;

/// Structure which is shared between the actix workers.
pub struct Config {
    /// Reference to the `sled` db, which is the local kv-store where dataset configs are saved.
    pub local_config: Db,
    /// HashMap which stores the metadata of datasets in memory, instead of constantly having to retrieve data from a storage backend.
    pub datasets: RwLock<HashMap<String, Dataset>>,
}
