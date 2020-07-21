use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use iterum_rust::pipeline::PipelineExecution;

// use crate::pipeline::models::PipelineResult;
use iterum_rust::vc::{Commit, Dataset};
use local::Local;
use serde::{Deserialize, Serialize};
// use storable::Storable;

pub mod local;
pub mod storable;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "backend", content = "credentials")]
pub enum Backend {
    Local(Local),
    AmazonS3,
    GoogleCloud,
}

// Dataset related:
impl Backend {
    pub fn store_committed_files(
        &self,
        dataset: &DatasetConfig,
        commit: &Commit,
        path: String,
    ) -> Result<(), std::io::Error> {
        match self {
            Backend::Local(backend) => backend.store_committed_files(dataset, commit, path),
            _ => panic!("Backend not implemented!"),
        }
    }
    pub fn get_file(&self, dataset_path: &str, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_file(dataset_path, commit_hash, filename),
            _ => unimplemented!(),
        }
    }

    pub fn save_dataset(&self, dataset_path: &str, dataset: &Dataset) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.save_dataset(dataset_path, dataset),
            _ => unimplemented!(),
        }
    }
    pub fn read_dataset(&self, dataset_path: &str) -> Result<Dataset, DaemonError> {
        match self {
            Backend::Local(backend) => backend.read_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }

    pub fn remove_dataset(&self, dataset_path: &str) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_dataset(dataset_path),
            _ => unimplemented!(),
        }
    }
}

// Pipeline related:
impl Backend {
    pub fn get_pipeline_executions(&self, dataset_path: &str) -> Result<Vec<String>, DaemonError> {
        match self {
            Backend::Local(backend) => backend.get_pipeline_executions(dataset_path),
            _ => unimplemented!(),
        }
    }

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

    pub fn remove_pipeline_execution(&self, dataset: &DatasetConfig, pipeline_hash: &str) -> Result<(), DaemonError> {
        match self {
            Backend::Local(backend) => backend.remove_pipeline_execution(dataset, pipeline_hash),
            _ => unimplemented!(),
        }
    }

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

    // pub fn get_pipeline_results(&self, dataset_path: &str, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
    //     match self {
    //         Backend::Local(backend) => backend.get_pipeline_results(dataset_path, pipeline_hash),
    //         _ => unimplemented!(),
    //     }
    // }
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
}
