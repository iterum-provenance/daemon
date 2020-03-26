use super::storable::Storable;
use crate::dataset::{Branch, Commit, Dataset, VersionTree};
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

// impl Local {
//     // pub fn new(path: &String) -> Local {
//     //     Local {
//     //         path: path.to_string(),
//     //     }
//     // }
// }

impl Storable for Local {
    fn store_committed_files(
        &self,
        dataset: &Dataset,
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
                fs::create_dir_all(&file_folder_path)
                    .expect("Could not create temporary file directory.");
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

    fn get_vtree(&self, dataset_path: &String) -> std::result::Result<VersionTree, DaemonError> {
        let path = format!("{}{}/vtree.json", self.path, dataset_path);
        let string = fs::read_to_string(path)?;
        let vtree = serde_json::from_str(&string)?;
        Ok(vtree)
    }

    fn set_vtree(
        &self,
        dataset_path: &String,
        vtree: &VersionTree,
    ) -> std::result::Result<(), DaemonError> {
        let vtree_string = serde_json::to_string_pretty(vtree)?;
        let path = format!("{}{}/vtree.json", self.path, dataset_path);
        let mut vtree_file = File::create(path)?;
        vtree_file.write_all(&vtree_string.as_bytes())?;
        Ok(())
    }

    fn get_dataset(&self, dataset_path: &String) -> std::result::Result<Dataset, DaemonError> {
        let path = format!("{}{}/dataset.json", self.path, dataset_path);
        let string = fs::read_to_string(&path)?;
        let dataset: Dataset = serde_json::from_str(&string)?;
        Ok(dataset)
    }

    fn create_dataset(&self, dataset: &Dataset) -> std::result::Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset.name);
        debug!("Path for dataset: {}", path);
        if std::path::Path::new(&path).exists() {
            Err(DaemonError::AlreadyExists)
        } else {
            debug!("trying to create a new dataset..");
            fs::create_dir_all(&path)?;
            fs::create_dir_all(format!("{}/data", &path))?;
            fs::create_dir_all(format!("{}/versions", &path))?;
            fs::create_dir_all(format!("{}/branches", &path))?;
            fs::create_dir_all(format!("{}/runs", &path))?;
            let string = serde_json::to_string_pretty(dataset)?;
            let mut dataset_file = File::create(format!("{}/dataset.json", path))?;
            dataset_file.write_all(&string.as_bytes())?;

            Ok(())
        }
    }

    fn remove_dataset(&self, dataset_path: &String) -> std::result::Result<(), DaemonError> {
        let path = format!("{}{}", self.path, dataset_path);
        fs::remove_dir_all(path)?;
        Ok(())
    }

    fn get_branch(
        &self,
        dataset_path: &String,
        branch_hash: &String,
    ) -> Result<Branch, DaemonError> {
        let path = format!(
            "{}{}/branches/{}.json",
            self.path, dataset_path, branch_hash
        );
        let string = fs::read_to_string(path)?;
        let branch = serde_json::from_str(&string)?;
        Ok(branch)
    }

    fn set_branch(
        &self,
        dataset_path: &std::string::String,
        branch: &Branch,
    ) -> Result<(), DaemonError> {
        let path = format!(
            "{}{}/branches/{}.json",
            self.path, dataset_path, branch.hash
        );
        let string = serde_json::to_string_pretty(&branch)?;
        let mut branch_file = File::create(path)?;
        branch_file.write_all(&string.as_bytes())?;
        Ok(())
    }

    fn get_commit(
        &self,
        dataset_path: &String,
        commit_hash: &String,
    ) -> Result<Commit, DaemonError> {
        let path = format!(
            "{}{}/versions/{}.json",
            self.path, dataset_path, commit_hash
        );
        let string = fs::read_to_string(path)?;
        let commit = serde_json::from_str(&string)?;
        Ok(commit)
    }

    fn create_commit(&self, dataset_path: &String, commit: &Commit) -> Result<(), DaemonError> {
        let path = format!(
            "{}{}/versions/{}.json",
            self.path, dataset_path, commit.hash
        );
        let string = serde_json::to_string_pretty(commit)?;
        let mut commit_file = File::create(path)?;
        commit_file.write_all(&string.as_bytes())?;
        Ok(())
    }

    fn get_file(
        &self,
        dataset_path: &String,
        commit_hash: &String,
        filename: &String,
    ) -> Result<Vec<u8>, DaemonError> {
        let file_path = format!(
            "{}{}/data/{}/{}",
            self.path, dataset_path, filename, commit_hash
        );
        match fs::read(&file_path) {
            Ok(contents) => Ok(contents),
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Err(DaemonError::NotFound),
                _ => Err(DaemonError::Io(error)),
            },
        }
    }

    fn save_vcdataset(&self, vcdataset: &VCDataset) -> Result<(), DaemonError> {
        // dataset.
        vcdataset
            .dataset
            .backend
            .create_dataset(&vcdataset.dataset)?;
        vcdataset
            .dataset
            .backend
            .set_vtree(&vcdataset.dataset.name, &vc_dataset.version_tree)?;
        dataset.backend.set_branch(&dataset.name, &master_branch)?;
        dataset.backend.create_commit(&dataset.name, &root_commit)?;

        let dataset_path = vcdataset.dataset.backend.path;
        let path = format!(
            "{}{}/versions/{}.json",
            self.path, dataset_path, commit.hash
        );
        let string = serde_json::to_string_pretty(commit)?;
        let mut commit_file = File::create(path)?;
        commit_file.write_all(&string.as_bytes())?;

        Ok(())
    }
}
