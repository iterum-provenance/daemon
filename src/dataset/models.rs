use crate::backend::Backend;
use crate::error::DaemonError;
use iterum_rust::vc::{Commit, Dataset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From<&DatasetConfig> for sled::IVec {
    fn from(dataset: &DatasetConfig) -> sled::IVec {
        debug!("Serializing struct {:?}", dataset);
        let string = serde_json::to_string(&dataset).expect("Serializing failed");
        string.into_bytes().into()
    }
}

impl From<sled::IVec> for DatasetConfig {
    fn from(ivec: sled::IVec) -> DatasetConfig {
        let string = String::from_utf8(ivec.to_vec()).expect("Converting bytes to string failed.");
        serde_json::from_str(&string).expect("Deserializing dataset failed")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatasetConfig {
    pub name: String,
    #[serde(flatten)]
    pub backend: Backend,
    pub description: String,
}
use crate::backend::storable::Storable;

impl DatasetConfig {
    pub fn store_committed_files(&self, commit: &Commit, path: String) -> Result<(), std::io::Error> {
        self.backend.store_committed_files(self, commit, path)
    }

    pub fn get_file(&self, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError> {
        self.backend.get_file(&self.name, commit_hash, filename)
    }

    pub fn save_dataset(&self, dataset: &Dataset) -> Result<(), DaemonError> {
        self.backend.save_dataset(&self.name, dataset)
    }

    pub fn read_dataset(&self) -> Result<Dataset, DaemonError> {
        self.backend.read_dataset(&self.name)
    }

    pub fn remove_dataset(&self) -> Result<(), DaemonError> {
        self.backend.remove_dataset(&self.name)
    }

    pub fn store_pipeline_result_files(
        &self,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        tmp_files_path: &str,
    ) -> Result<(), std::io::Error> {
        self.backend
            .store_pipeline_result_files(self, pipeline_result_paths, pipeline_hash, tmp_files_path)
    }

    // pub fn get_pipeline_results(&self, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
    //     self.backend.get_pipeline_results(&self.name, pipeline_hash)
    // }

    pub fn get_pipeline_result(&self, pipeline_hash: &str, file_name: &str) -> Result<Vec<u8>, DaemonError> {
        self.backend.get_pipeline_result(&self.name, pipeline_hash, file_name)
    }
}
