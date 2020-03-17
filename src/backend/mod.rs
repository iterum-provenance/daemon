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
    fn store_commit_files(
        &self,
        dataset: &Dataset,
        path: String,
    ) -> Result<Commit, std::io::Error> {
        match self {
            Backend::Local(backend) => backend.store_commit_files(dataset, path),
            _ => panic!("Backend not implemented!"),
        }
    }
}
