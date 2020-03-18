use sled::Db;

#[derive(Clone)]
pub struct Config {
    pub app_name: String,
    pub storage_path: String,
    pub dataset_path: String,
    pub cache: Db,
}
