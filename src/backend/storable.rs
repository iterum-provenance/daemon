use crate::commit::Commit;
use crate::dataset::Dataset;
pub trait Storable {
    // Trait for backends which is used to store the types to the backend.
    fn store_committed_files(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error>;

    fn get_commit_from_file(&self, path: String) -> Result<Commit, std::io::Error>;
}
