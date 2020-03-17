use crate::commit::Commit;
use crate::dataset::Dataset;
pub trait Storable {
    // Trait for backends which is used to store the types to the backend.
    fn store_commit_files(&self, dataset: &Dataset, path: String)
        -> Result<Commit, std::io::Error>;
}
