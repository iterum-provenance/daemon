use super::dataset::VCDataset;
use super::error::VersionControlError;
use crate::pipeline::models::PipelineResult;

impl VCDataset {
    pub fn add_pipeline_result(
        mut self,
        pipeline_result: &PipelineResult,
    ) -> Result<VCDataset, VersionControlError> {
        // Check whether the hash does not already exist:
        if self.pipeline_results.contains_key(&pipeline_result.hash) {
            return Err(VersionControlError::PipelineHashAlreadyExists);
        }

        self.pipeline_results
            .insert(pipeline_result.hash.to_string(), pipeline_result.clone());

        Ok(self)
    }

    pub fn set_pipeline_result(
        mut self,
        pipeline_result: &PipelineResult,
    ) -> Result<VCDataset, VersionControlError> {
        self.pipeline_results
            .insert(pipeline_result.hash.to_string(), pipeline_result.clone());

        Ok(self)
    }
}
