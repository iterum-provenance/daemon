use super::Local;
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use iterum_rust::pipeline::PipelineExecution;
use iterum_rust::provenance::FragmentLineage;

use std::fs;
use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

impl Local {
    fn get_pipelines_path(&self, dataset_name: &str) -> PathBuf {
        let file_dir = format!("{}{}/runs", self.path, dataset_name);
        let file_folder_path = std::path::Path::new(&file_dir);
        file_folder_path.to_owned()
    }

    fn get_pipeline_path(&self, dataset_name: &str, pipeline_hash: &str) -> PathBuf {
        let file_dir = self.get_pipelines_path(&dataset_name);
        file_dir.join(&pipeline_hash)
    }

    pub fn get_pipeline_executions(&self, dataset_path: &str) -> Result<Vec<String>, DaemonError> {
        let path = self.get_pipelines_path(dataset_path);

        let files: Vec<String> = fs::read_dir(path)?
            .map(|direntry| direntry.unwrap().file_name().to_str().unwrap().into())
            .collect();

        Ok(files)
    }

    pub fn get_pipeline_execution(
        &self,
        dataset_path: &str,
        pipeline_hash: &str,
    ) -> Result<PipelineExecution, DaemonError> {
        let path = self.get_pipeline_path(dataset_path, pipeline_hash);

        let path = path.join("execution.json");

        let string = fs::read_to_string(path)?;
        let pipeline_execution: PipelineExecution = serde_json::from_str(&string)?;

        Ok(pipeline_execution)
    }

    pub fn store_pipeline_execution(
        &self,
        dataset: &DatasetConfig,
        pipeline_execution: &PipelineExecution,
    ) -> Result<(), DaemonError> {
        let path = self.get_pipeline_path(&dataset.name, &pipeline_execution.pipeline_run.pipeline_run_hash);
        if !path.exists() {
            fs::create_dir_all(&path).expect("Could not create pipeline directory.");
        }
        let execution_file = path.join("execution.json");
        let string = serde_json::to_string_pretty(pipeline_execution)?;
        let mut file = File::create(execution_file)?;
        file.write_all(&string.as_bytes())?;

        Ok(())
    }

    pub fn remove_pipeline_execution(&self, dataset: &DatasetConfig, pipeline_hash: &str) -> Result<(), DaemonError> {
        let path = self.get_pipeline_path(&dataset.name, &pipeline_hash);
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    pub fn store_pipeline_result_files(
        &self,
        dataset: &DatasetConfig,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        _tmp_files_path: &str,
    ) -> Result<(), std::io::Error> {
        for file in pipeline_result_paths {
            let (filename, filepath) = file;
            let path = self.get_pipeline_path(&dataset.name, &pipeline_hash).join("results");
            if !path.exists() {
                fs::create_dir_all(&path).expect("Could not create pipeline directory.");
            }
            let new_filepath = path.join(filename);
            fs::copy(&filepath, &new_filepath)?;
        }

        Ok(())
    }

    pub fn get_pipeline_results(&self, dataset_path: &str, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
        let path = self.get_pipeline_path(dataset_path, &pipeline_hash).join("results");
        let files: Vec<String> = fs::read_dir(path)?
            .map(|direntry| direntry.unwrap().file_name().to_str().unwrap().into())
            .collect();

        Ok(files)
    }

    pub fn get_pipeline_result(
        &self,
        dataset_path: &str,
        pipeline_hash: &str,
        file_name: &str,
    ) -> Result<Vec<u8>, DaemonError> {
        let path = self.get_pipeline_path(dataset_path, &pipeline_hash).join("results");
        let file_path = path.join(file_name);

        match fs::read(&file_path) {
            Ok(contents) => Ok(contents),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Err(DaemonError::NotFound),
                _ => Err(DaemonError::Io(error)),
            },
        }
    }

    pub fn store_pipeline_fragment_lineage(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
        fragment: &FragmentLineage,
    ) -> Result<(), DaemonError> {
        let path = self.get_pipeline_path(&dataset.name, &pipeline_hash).join("lineage");
        if !path.exists() {
            fs::create_dir_all(&path).expect("Could not create lineage directory.");
        }
        let new_filepath = path.join(&fragment.description.metadata.fragment_id);
        let string = serde_json::to_string_pretty(fragment)?;
        let mut file = File::create(new_filepath)?;
        file.write_all(&string.as_bytes())?;
        Ok(())
    }

    pub fn get_pipeline_fragment_lineages(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
    ) -> Result<Vec<String>, DaemonError> {
        let path = self.get_pipeline_path(&dataset.name, &pipeline_hash).join("lineage");

        let fragments: Vec<String> = fs::read_dir(path)?
            .map(|direntry| direntry.unwrap().file_name().to_str().unwrap().into())
            .collect();

        Ok(fragments)
    }

    pub fn get_pipeline_fragment_lineage(
        &self,
        dataset: &DatasetConfig,
        pipeline_hash: &str,
        fragment_id: &str,
    ) -> Result<FragmentLineage, DaemonError> {
        let path = self.get_pipeline_path(&dataset.name, &pipeline_hash).join("lineage");
        let file_path = path.join(fragment_id);

        let string = fs::read_to_string(file_path)?;
        let fragment_lineage: FragmentLineage = serde_json::from_str(&string)?;
        Ok(fragment_lineage)
    }
}
