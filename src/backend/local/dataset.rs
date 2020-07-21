use super::Local;
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use iterum_rust::vc::{Commit, Dataset};
use std::fs;
use std::fs::File;
use std::io::Write;

impl Local {
    pub fn store_committed_files(
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

    pub fn get_file(&self, dataset_path: &str, commit_hash: &str, filename: &str) -> Result<Vec<u8>, DaemonError> {
        let file_path = format!("{}{}/data/{}/{}", self.path, dataset_path, filename, commit_hash);
        match fs::read(&file_path) {
            Ok(contents) => Ok(contents),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Err(DaemonError::NotFound),
                _ => Err(DaemonError::Io(error)),
            },
        }
    }

    pub fn save_dataset(&self, dataset_path: &str, dataset: &Dataset) -> Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset_path);
        debug!("Path for dataset: {}", path);
        if !std::path::Path::new(&path).exists() {
            fs::create_dir_all(&path)?;
        }
        debug!("trying to create a new dataset..");
        fs::create_dir_all(&path)?;
        fs::create_dir_all(format!("{}/data", &path))?;
        let string = serde_json::to_string_pretty(dataset)?;
        let mut dataset_file = File::create(format!("{}/dataset.json", path))?;
        dataset_file.write_all(&string.as_bytes())?;

        Ok(())
    }

    pub fn read_dataset(&self, dataset_path: &str) -> Result<Dataset, DaemonError> {
        let path = format!("{}{}/dataset.json", self.path, dataset_path);

        let string = fs::read_to_string(path)?;
        let dataset: Dataset = serde_json::from_str(&string)?;

        Ok(dataset)
    }

    pub fn remove_dataset(&self, dataset_path: &str) -> Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset_path);
        match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}
