use sled::Db;

pub struct Config {
    pub cache: Db,
}
