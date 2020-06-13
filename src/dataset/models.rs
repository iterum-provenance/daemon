use crate::backend::Backend;
use crate::error::DaemonError;
use crate::version_control::dataset::VCDataset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Added,
    Removed,
    Updated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub updated: Vec<String>, // pub change_type: ChangeType,
                              // pub files: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deprecated {
    pub value: bool,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub parent: Option<String>,
    pub branch: String,
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
    pub diff: Diff,
    pub deprecated: Deprecated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, String>,
}

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

    pub fn save_vcdataset(&self, dataset: &VCDataset) -> Result<(), DaemonError> {
        self.backend.save_vcdataset(&self.name, dataset)
    }

    pub fn read_vcdataset(&self) -> Result<VCDataset, DaemonError> {
        self.backend.read_vcdataset(&self.name)
    }

    pub fn remove_vcdataset(&self) -> Result<(), DaemonError> {
        self.backend.remove_vcdataset(&self.name)
    }

    pub fn store_pipeline_result_files(
        &self,
        dataset: &DatasetConfig,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        tmp_files_path: &str,
    ) -> Result<(), std::io::Error> {
        self.backend
            .store_pipeline_result_files(self, pipeline_result_paths, pipeline_hash, tmp_files_path)
    }

    pub fn get_pipeline_results(&self, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
        self.backend.get_pipeline_results(&self.name, pipeline_hash)
    }
}
