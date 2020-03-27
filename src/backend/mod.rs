use crate::dataset::{Commit, Dataset};
use crate::error::DaemonError;

use crate::version_control::dataset::VCDataset;
use local::Local;
use serde::{Deserialize, Serialize};
use storable::Storable;

pub mod local;
pub mod storable;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "backend", content = "credentials")]
pub enum Backend {
    Local(Local),
    AmazonS3,
    GoogleCloud,
}

impl Storable for Backend {
    fn store_committed_files(
        &self,
        dataset: &Dataset,
        commit: &Commit,
        path: String,
    ) -> Result<(), std::io::Error> {
        match self {
            Backend::Local(backend) => backend.store_committed_files(dataset, commit, path),
            _ => panic!("Backend not implemented!"),
        }
    }

    fn get_file(
        &self,
        dataset_path: &str,
        commit_hash: &str,
        filename: &str,
    ) -> Result<Vec<u8>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_file(dataset_path, commit_hash, filename),
            _ => unimplemented!(),
        }
    }

    fn save_vcdataset(&self, dataset_path: &str, dataset: &VCDataset) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.save_vcdataset(dataset_path, dataset),
            _ => unimplemented!(),
        }
    }
    fn read_vcdataset(&self, dataset_path: &str) -> Result<VCDataset, DaemonError> {
        match self {
            Backend::Local(backend) => backend.read_vcdataset(dataset_path),
            _ => unimplemented!(),
        }
    }

    fn remove_vcdataset(&self, dataset_path: &str) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_vcdataset(dataset_path),
            _ => unimplemented!(),
        }
    }
}
