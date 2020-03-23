use sled::Db;

#[derive(Clone)]
pub struct Config {
    pub cache: Db,
}
