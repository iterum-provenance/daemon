use crate::dataset::{Commit, Dataset};
use crate::error::DaemonError;
use crate::version_control::dataset::VCDataset;

pub trait Storable {
    /// Trait for backends which is used to store the types to the backend.
    /// These functions simply store the structs in the format the backend requires.
    /// It does not perform any integrity checks. This should already have been done at this point.
    fn store_committed_files(
        &self,
        dataset: &Dataset,
        commit: &Commit,
        tmp_files_path: String,
    ) -> Result<(), std::io::Error>;
    fn get_file(
        &self,
        dataset_path: &str,
        commit_hash: &str,
        filename: &str,
    ) -> Result<Vec<u8>, DaemonError>;
    fn save_vcdataset(&self, dataset_path: &str, dataset: &VCDataset) -> Result<(), DaemonError>;
    fn read_vcdataset(&self, dataset_path: &str) -> Result<VCDataset, DaemonError>;
    fn remove_vcdataset(&self, dataset_path: &str) -> Result<(), DaemonError>;

    fn store_pipeline_result_files(
        &self,
        dataset: &Dataset,
        pipeline_result_hash: &str,
        tmp_files_path: &str,
    ) -> Result<(), std::io::Error>;
}
