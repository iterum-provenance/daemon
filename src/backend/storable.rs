use crate::backend::error::BackendError;
use crate::commit::Commit;
use crate::dataset::{Branch, Dataset, VersionTree};

pub trait Storable {
    /// Trait for backends which is used to store the types to the backend.
    /// These functions simply store the structs in the format the backend requires.
    /// It does not perform any integrity checks. This should already have been done at this point.
    fn store_committed_files(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error>;

    fn get_commit_from_file(&self, path: String) -> Result<Commit, std::io::Error>;

    fn get_vtree(&self, dataset_path: &String) -> Result<VersionTree, BackendError>;
    fn set_vtree(&self, dataset_path: &String, vtree: &VersionTree) -> Result<(), BackendError>;
    fn get_dataset(&self, dataset_path: &String) -> Result<Dataset, BackendError>;
    fn create_dataset(&self, dataset: &Dataset) -> Result<(), BackendError>;
    fn remove_dataset(&self, dataset_path: &String) -> Result<(), BackendError>;
    fn get_branch(
        &self,
        dataset_path: &String,
        branch_hash: &String,
    ) -> Result<Branch, BackendError>;
    fn set_branch(&self, dataset_path: &String, branch: &Branch) -> Result<(), BackendError>;
    fn get_commit(
        &self,
        dataset_path: &String,
        commit_hash: &String,
    ) -> Result<Commit, BackendError>;
    fn create_commit(&self, dataset_path: &String, commit: &Commit) -> Result<(), BackendError>;
}
