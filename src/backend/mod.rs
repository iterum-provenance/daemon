//! Module which represents the storage interface for Iterum. It contains the logic necessary to connect to different storage backends, though currently only the LocalStorage backend is implemented.
//! Different storage backends can be implemented by implementing the functions for the other Enum variants.
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use iterum_rust::pipeline::PipelineExecution;
use iterum_rust::provenance::FragmentLineage;
use iterum_rust::vc::{Commit, Dataset};
use local::Local;
use serde::{Deserialize, Serialize};

pub mod local;

/// Various types of backends. Only Local has been implemented. The content of the internal struct contain the `credentials` for the backend. For AmazonS3 this could be an API key for example.
/// On this enum, various functions are implemented which the Daemon requires. The implemented functions redirect to the corresponding struct, so to support more backends, the functions need to be implemented on these structs.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "backend", content = "credentials")]
pub enum Backend {
    Local(Local),
    AmazonS3,
    GoogleCloud,
}

// Dataset related
impl Backend {
    /// Describes how commited files should be stored in the backend.
    pub fn store_committed_files(
        &self,
        dataset: &DatasetConfig,
        commit: &Commit,
        path: String,
    ) -> Result<(), std::io::Error> {
        match self {
            Backend::Local(backend) => backend.store_committed_files(dataset, commit, path),
            _ => unimplemented!(),
        }
    }

    /// Describes how to retrieve a file from the dataset.
    pub fn get_file(&self, dataset_path: &str, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_file(dataset_path, commit_hash, filename),
            _ => unimplemented!(),
        }
    }

    /// Describes how to save a dataset struct (which is the metadata/version info of a dataset, not the data itself).
    pub fn save_dataset(&self, dataset_path: &str, dataset: &Dataset) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.save_dataset(dataset_path, dataset),
            _ => unimplemented!(),
        }
    }
    /// Describes how to retrieve a dataset struct from the storage backend.
    pub fn read_dataset(&self, dataset_path: &str) -> Result<Dataset, DaemonError> {
        match self {
            Backend::Local(backend) => backend.read_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }

    /// Describes how to remove a dataset as a whole from the storage backend.
    pub fn remove_dataset(&self, dataset_path: &str) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }
}

// Pipeline related:
impl Backend {
    /// Describes how to retrieve pipeline executions for a dataset from the storage backend.
    pub fn get_pipeline_executions(&self, dataset_path: &str) -> Result<Vec<String>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_executions(dataset_path),
            _ => unimplemented!(),
        }
    }

    /// Describes how to retrieve a specific pipeline execution from the storage backend.
    pub fn get_pipeline_execution(
        &self,
        dataset_path: &str,
        pipeline_hash: &str,
    ) -> Result<PipelineExecution, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_execution(dataset_path, pipeline_hash),
            _ => unimplemented!(),
        }
    }

    /// Describes how to store a pipeline execution in the storage backend.
    pub fn store_pipeline_execution(
        &self,
        dataset: &DatasetConfig,
        pipeline_execution: &PipelineExecution,
    ) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.store_pipeline_execution(dataset, pipeline_execution),
            _ => unimplemented!(),
        }
    }

    /// Describes how to remove a specific pipeline execution from the storage backend.
    pub fn remove_pipeline_execution(&self, dataset: &DatasetConfig, pipeline_hash: &str) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_pipeline_execution(dataset, pipeline_hash),
            _ => unimplemented!(),
        }
    }

    /// Describes how to store results of a pipeline in the storage backend.
    pub fn store_pipeline_result_files(
        &self,
        dataset: &DatasetConfig,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        tmp_files_path: &str,
    ) -> Result<(), std::io::Error> {
        match self {
            Backend::Local(backend) => {
                backend.store_pipeline_result_files(dataset, pipeline_result_paths, pipeline_hash, tmp_files_path)
            }
            _ => unimplemented!(),
        }
    }

    /// Describes how to retrieve a list of results of a pipeline in the storage backend. Returns a list of filenames, not the data itself
    pub fn get_pipeline_results(&self, dataset_path: &str, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_results(dataset_path, pipeline_hash),
            _ => unimplemented!(),
        }
    }

    /// Describes how to get a specific pipeline result from the storage backend. Returns actual data.
    pub fn get_pipeline_result(
        &self,
        dataset_path: &str,
        pipeline_hash: &str,
        file_name: &str,
    ) -> Result<Vec<u8>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_result(dataset_path, pipeline_hash, file_name),
            _ => unimplemented!(),
        }
    }

    /// Describes how to store a FragmentLineage from a pipeline in the storage backend.
    pub fn store_pipeline_fragment_lineage(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
        fragment: &FragmentLineage,
    ) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.store_pipeline_fragment_lineage(dataset, pipeline_hash, fragment),
            _ => unimplemented!(),
        }
    }

    /// Describes how to retrieve all lineage information from the storage backend. Returns a list of fragment hashes.
    pub fn get_pipeline_fragment_lineages(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
    ) -> Result<Vec<String>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_fragment_lineages(dataset, pipeline_hash),
            _ => unimplemented!(),
        }
    }

    /// Describes how to retrieve a specific FragmentLineage from the storage backend.
    pub fn get_pipeline_fragment_lineage(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
        fragment_id: &str,
    ) -> Result<FragmentLineage, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_fragment_lineage(dataset, pipeline_hash, fragment_id),
            _ => unimplemented!(),
        }
    }
}
