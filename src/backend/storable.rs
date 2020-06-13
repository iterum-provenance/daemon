use crate::dataset::{Commit, DatasetConfig};
use crate::error::DaemonError;
// use crate::pipeline::models::PipelineResult;
use crate::version_control::dataset::VCDataset;

pub trait Storable {
    /// Trait for backends which is used to store the types to the backend.
    /// These functions simply store the structs in the format the backend requires.
    /// It does not perform any integrity checks. This should already have been done at this point.
    fn store_committed_files(
        &self,
        dataset: &DatasetConfig,
        commit: &Commit,
        tmp_files_path: String,
    ) -> Result<(), std::io::Error>;
    fn get_file(&self, dataset_path: &str, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError>;
    fn save_vcdataset(&self, dataset_path: &str, dataset: &VCDataset) -> Result<(), DaemonError>;
    fn read_vcdataset(&self, dataset_path: &str) -> Result<VCDataset, DaemonError>;
    fn remove_vcdataset(&self, dataset_path: &str) -> Result<(), DaemonError>;

    fn store_pipeline_result_files(
        &self,
        dataset: &DatasetConfig,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        tmp_files_path: &str,
    ) -> Result<(), std::io::Error>;
    fn get_pipeline_results(&self, dataset_path: &str, pipeline_hash: &str) -> Result<Vec<String>, DaemonError>;
}
