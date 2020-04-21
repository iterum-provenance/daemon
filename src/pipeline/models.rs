use crate::dataset::Dataset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineResult {
    pub hash: String,
    pub dataset_hash: String,
    pub commit_hash: String,
    pub files: Vec<String>,
}
