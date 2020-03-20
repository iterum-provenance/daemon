use super::storable::Storable;
use crate::dataset::{Branch, ChangeType, Commit, Dataset, VersionTree};
use crate::error::DaemonError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Local {
    pub path: String,
}

impl Local {
    // pub fn new(path: &String) -> Local {
    //     Local {
    //         path: path.to_string(),
    //     }
    // }
}

impl Storable for Local {
    fn store_committed_files(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error> {
        debug!("Storing commit in backend.");

        let config_path = format!("{}commit.json", path);
        let commit_string = fs::read_to_string(config_path)?;
        let commit: Commit = serde_json::from_str(&commit_string)?;

        // // Create the new files wherever necessary
        for item in &commit.diff {
            match item.change_type {
                ChangeType::Add => {
                    debug!("Adding files with names:");
                    for file in &item.files {
                        let tmp_file_path = format!("{}{}", &path, file);
                        debug!("Pulling file from: {}", tmp_file_path);

                        let file_dir = format!("{}{}/data/{}", self.path, dataset.name, file);
                        fs::create_dir_all(&file_dir)
                            .expect("Could not create data file directory.");

                        let file_path = format!("{}/{}", &file_dir, commit.hash);
                        debug!("Storing file in: {}", file_path);
                        fs::copy(&tmp_file_path, &file_path)?;
                    }
                }
                ChangeType::Remove => {}
                ChangeType::Update => {}
            }
        }

        Ok(())
    }

    fn get_commit_from_file(&self, path: String) -> Result<Commit, std::io::Error> {
        let config_path = format!("{}commit.json", path);
        let commit_string = fs::read_to_string(config_path)?;
        let commit: Commit = serde_json::from_str(&commit_string)?;

        Ok(commit)
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
            fs::create_dir_all(&path)?;
            fs::create_dir_all(format!("{}/data", &path))?;
            fs::create_dir_all(format!("{}/versions", &path))?;
            fs::create_dir_all(format!("{}/branches", &path))?;
            fs::create_dir_all(format!("{}/runs", &path))?;
            let string = serde_json::to_string_pretty(dataset)?;
            let mut dataset_file = File::create(format!("{}/dataset.json", path))?;
            dataset_file.write_all(&string.as_bytes())?;
            let tree = HashMap::new();
            let branches = HashMap::new();
            let vtree = VersionTree {
                tree: tree,
                branches: branches,
            };
            self.set_vtree(&dataset.name, &vtree)?;
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
}
