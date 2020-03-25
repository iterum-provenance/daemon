use crate::version_control::dataset::VCDataset;
use sled::Db;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Config {
    pub cache: Db,
    pub state: HashMap<String, RwLock<VCDataset>>,
}
