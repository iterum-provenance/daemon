use crate::commit::Commit;
use crate::dataset::Dataset;
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
            _ => panic!("Backend not implemented!"),
        }
    }
}
