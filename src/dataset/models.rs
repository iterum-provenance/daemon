use crate::backend::Backend;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub backend: Backend,
    pub description: String,
}

impl Dataset {
    pub fn read_from_path(path: &String) -> Result<Dataset, Box<Error>> {
        let dataset_path = format!("{}/dataset.json", &path);
        let string: String = fs::read_to_string(&dataset_path)?;
        let item: Dataset = serde_json::from_str(&string)?;
        Ok(item)
    }

    // fn find_by_path(backend: &String, path: &String) -> Result<Dataset, Box<Error>> {
    //     let config_path = format!("{}/{}/dataset.json", backend, path);
    //     let string = fs::read_to_string(&config_path)?;
    //     serde_json::from_str(&string)?
    // }
}
