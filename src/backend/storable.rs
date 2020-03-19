use crate::dataset::{Branch, Commit, Dataset, VersionTree};
use crate::error::DaemonError;

pub trait Storable {
    /// Trait for backends which is used to store the types to the backend.
    /// These functions simply store the structs in the format the backend requires.
    /// It does not perform any integrity checks. This should already have been done at this point.
    fn store_committed_files(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error>;

    fn get_commit_from_file(&self, path: String) -> Result<Commit, std::io::Error>;

    fn get_vtree(&self, dataset_path: &String) -> Result<VersionTree, DaemonError>;
    fn set_vtree(&self, dataset_path: &String, vtree: &VersionTree) -> Result<(), DaemonError>;
    fn get_dataset(&self, dataset_path: &String) -> Result<Dataset, DaemonError>;
    fn create_dataset(&self, dataset: &Dataset) -> Result<(), DaemonError>;
    fn remove_dataset(&self, dataset_path: &String) -> Result<(), DaemonError>;
    fn get_branch(
        &self,
        dataset_path: &String,
        branch_hash: &String,
    ) -> Result<Branch, DaemonError>;
    fn set_branch(&self, dataset_path: &String, branch: &Branch) -> Result<(), DaemonError>;
    fn get_commit(
        &self,
        dataset_path: &String,
        commit_hash: &String,
    ) -> Result<Commit, DaemonError>;
    fn create_commit(&self, dataset_path: &String, commit: &Commit) -> Result<(), DaemonError>;
    fn get_file(
        &self,
        dataset_path: &String,
        commit_hash: &String,
        filename: &String,
    ) -> Result<Vec<u8>, DaemonError>;
}
