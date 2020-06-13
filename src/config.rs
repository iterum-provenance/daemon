use crate::dataset::DatasetConfig;
use crate::version_control::dataset::VCDataset;
use sled::Db;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Config {
    pub local_config: Db,
    // pub dataset_configs: RwLock<HashMap<String, DatasetConfig>>,
    pub datasets: RwLock<HashMap<String, VCDataset>>,
}
