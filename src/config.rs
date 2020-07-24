//! Module which contains storage which is shared by the different endpoint handlers. It has a reference to the `local_config` which is a key-value store used to store DatasetConfigs, but also
//! a HashMap where the available data sets are stored in memory, for quicker access.
use iterum_rust::vc::Dataset;
use sled::Db;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Config {
    pub local_config: Db,
    // pub dataset_configs: RwLock<HashMap<String, DatasetConfig>>,
    pub datasets: RwLock<HashMap<String, Dataset>>,
}
