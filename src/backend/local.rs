use super::storable::Storable;
use crate::dataset::{Commit, DatasetConfig};
use crate::error::DaemonError;
use crate::version_control::dataset::VCDataset;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Local {
    pub path: String,
}

impl Storable for Local {
    fn store_committed_files(
        &self,
        dataset: &DatasetConfig,
        commit: &Commit,
        tmp_files_path: String,
    ) -> Result<(), std::io::Error> {
        debug!("Storing commit in backend.");

        // // Create the new files wherever necessary
        debug!("Adding files with names:");
        for file in &commit.diff.added {
            let tmp_file_path = format!("{}/{}", &tmp_files_path, file);
            debug!("Pulling file from: {}", tmp_file_path);

            let file_dir = format!("{}{}/data/{}", self.path, dataset.name, file);
            // fs::create_dir_all(&file_dir).expect("Could not create data file directory.");
            let file_folder_path = std::path::Path::new(&file_dir).parent().unwrap();
            if !file_folder_path.exists() {
                fs::create_dir_all(&file_folder_path).expect("Could not create temporary file directory.");
            }
            debug!("Storing file in: {}", file_dir);
            fs::copy(&tmp_file_path, &file_dir)?;
        }
        for file in &commit.diff.updated {
            let tmp_file_path = format!("{}/{}", &tmp_files_path, file);
            debug!("Pulling file from: {}", tmp_file_path);

            let file_dir = format!("{}{}/data/{}", self.path, dataset.name, file);
            debug!("Storing file in: {}", file_dir);
            fs::copy(&tmp_file_path, &file_dir)?;
        }

        Ok(())
    }

    fn get_file(&self, dataset_path: &str, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError> {
        let file_path = format!("{}{}/data/{}/{}", self.path, dataset_path, filename, commit_hash);
        match fs::read(&file_path) {
            Ok(contents) => Ok(contents),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Err(DaemonError::NotFound),
                _ => Err(DaemonError::Io(error)),
            },
        }
    }

    fn save_vcdataset(&self, dataset_path: &str, vcdataset: &VCDataset) -> Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset_path);
        debug!("Path for dataset: {}", path);
        if !std::path::Path::new(&path).exists() {
            fs::create_dir_all(&path)?;
        }
        debug!("trying to create a new dataset..");
        fs::create_dir_all(&path)?;
        fs::create_dir_all(format!("{}/data", &path))?;
        let string = serde_json::to_string_pretty(vcdataset)?;
        let mut dataset_file = File::create(format!("{}/vcdataset.json", path))?;
        dataset_file.write_all(&string.as_bytes())?;

        Ok(())
    }

    fn read_vcdataset(&self, dataset_path: &str) -> Result<VCDataset, DaemonError> {
        let path = format!("{}{}/vcdataset.json", self.path, dataset_path);

        let string = fs::read_to_string(path)?;
        let vcdataset: VCDataset = serde_json::from_str(&string)?;

        Ok(vcdataset)
    }

    fn remove_vcdataset(&self, dataset_path: &str) -> Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset_path);
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    fn store_pipeline_result_files(
        &self,
        dataset: &DatasetConfig,
        pipeline_result_paths: &[(String, String)],
        pipeline_hash: &str,
        _tmp_files_path: &str,
    ) -> Result<(), std::io::Error> {
        debug!("Adding files with names:");
        for file in pipeline_result_paths {
            let (filename, filepath) = file;
            debug!("Pulling file from: {}", filepath);

            let file_dir = format!("{}{}/runs/{}", self.path, dataset.name, pipeline_hash);
            let file_folder_path = std::path::Path::new(&file_dir);
            debug!("Dir path: {:?}", file_folder_path);
            if !file_folder_path.exists() {
                fs::create_dir_all(&file_folder_path).expect("Could not create temporary file directory.");
            }
            let new_filepath = format!("{}/{}", file_dir, filename);
            debug!("Storing file in: {}", new_filepath);
            fs::copy(&filepath, &new_filepath)?;
        }

        Ok(())
    }

    fn get_pipeline_results(&self, dataset_path: &str, pipeline_hash: &str) -> Result<Vec<String>, DaemonError> {
        let path = format!("{}{}/runs/{}", self.path, dataset_path, pipeline_hash);
        let files: Vec<String> = fs::read_dir(path)?
            .map(|direntry| direntry.unwrap().path().to_str().unwrap().into())
            .collect();

        Ok(files)
    }
}
