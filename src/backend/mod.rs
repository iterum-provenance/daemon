use crate::dataset::{Branch, Commit, Dataset, VersionTree};
use crate::error::DaemonError;

use local::Local;
use serde::{Deserialize, Serialize};
use storable::Storable;

pub mod local;
pub mod storable;

#[derive(Serialize, Deserialize, Debug)]
pub enum Backend {
    Local(Local),
    AmazonS3,
    GoogleCloud,
}

impl Storable for Backend {
    fn store_committed_files(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error> {
        match self {
            Backend::Local(backend) => backend.store_committed_files(dataset, path),
            _ => panic!("Backend not implemented!"),
        }
    }

    fn get_commit_from_file(&self, path: String) -> Result<Commit, std::io::Error> {
        match self {
            Backend::Local(backend) => backend.get_commit_from_file(path),
            _ => unimplemented!(),
        }
    }

    fn get_vtree(&self, dataset_path: &String) -> Result<VersionTree, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_vtree(dataset_path),
            _ => unimplemented!(),
        }
    }

    fn set_vtree(&self, dataset_path: &String, vtree: &VersionTree) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.set_vtree(dataset_path, vtree),
            _ => unimplemented!(),
        }
    }

    fn get_dataset(&self, dataset_path: &String) -> Result<Dataset, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }

    fn create_dataset(&self, dataset: &Dataset) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.create_dataset(dataset),
            _ => unimplemented!(),
        }
    }

    fn remove_dataset(&self, dataset_path: &String) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }

    fn get_branch(
        &self,
        dataset_path: &String,
        branch_hash: &String,
    ) -> Result<Branch, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_branch(dataset_path, branch_hash),
            _ => unimplemented!(),
        }
    }

    fn set_branch(&self, dataset_path: &String, branch: &Branch) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.set_branch(dataset_path, branch),
            _ => unimplemented!(),
        }
    }

    fn get_commit(
        &self,
        dataset_path: &String,
        commit_hash: &String,
    ) -> Result<Commit, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_commit(dataset_path, commit_hash),
            _ => unimplemented!(),
        }
    }

    fn create_commit(&self, dataset_path: &String, commit: &Commit) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.create_commit(dataset_path, commit),
            _ => unimplemented!(),
        }
    }

    fn get_file(
        &self,
        dataset_path: &String,
        commit_hash: &String,
        filename: &String,
    ) -> Result<Vec<u8>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_file(dataset_path, commit_hash, filename),
            _ => unimplemented!(),
        }
    }
}
