use crate::backend::Backend;
use crate::commit;
use crate::commit::{ChangeType, Commit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, Branch>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub backend: Backend,
    pub description: String,
}

impl Dataset {
    pub fn read_from_path(path: &String) -> Result<Dataset, Box<Error>> {
        let dataset_path = format!("{}/dataset.json", &path);
        let string: String = fs::read_to_string(&dataset_path)?;
        let item: Dataset = serde_json::from_str(&string)?;
        Ok(item)
    }

    pub fn add_commit(&self, tmp_path: &String, commit: commit::Commit) -> commit::Commit {
        let dataset_path = match &self.backend {
            Backend::Local(backend) => format!("{}{}", backend.path, self.path),
            _ => panic!("Backend not implemented"),
        };

        // Create commit file
        let commit_string = serde_json::to_string_pretty(&commit).unwrap();
        let mut commit_file =
            File::create(format!("{}/versions/{}.json", dataset_path, commit.hash)).unwrap();
        commit_file.write_all(&commit_string.as_bytes()).unwrap();

        // Update version tree
        let mut version_tree = self.get_vtree().unwrap();
        version_tree.tree.insert(
            commit.hash.clone(),
            VersionTreeNode {
                name: "".to_string(),
                branch: commit.branch.clone(),
                children: vec![],
                parent: None,
            },
        );
        let vtree_string = serde_json::to_string_pretty(&version_tree).unwrap();
        let mut vtree_file = File::create(format!("{}/vtree.json", dataset_path)).unwrap();
        vtree_file.write_all(&vtree_string.as_bytes()).unwrap();

        // // Create the new files wherever necessary
        for item in &commit.diff {
            match item.change_type {
                ChangeType::Add => {
                    debug!("Adding files with names:");
                    for file in &item.files {
                        let tmp_file_path = format!("{}{}", &tmp_path, file);
                        debug!("Pulling file from: {}", tmp_file_path);

                        let file_dir = format!("{}/data/{}", dataset_path, file);
                        fs::create_dir_all(&file_dir)
                            .expect("Could not create data file directory.");

                        let file_path = format!("{}/{}", &file_dir, commit.hash);
                        debug!("Storing file in: {}", file_path);
                        fs::copy(&tmp_file_path, &file_path).unwrap();
                    }
                }
                ChangeType::Remove => {}
                ChangeType::Update => {}
            }
        }
        commit
    }

    pub fn get_vtree(&self) -> Result<VersionTree, Box<Error>> {
        let vtree_path = match &self.backend {
            Backend::Local(backend) => format!("{}{}/vtree.json", backend.path, self.path),
            _ => panic!("Backend not implemented"),
        };
        let string = fs::read_to_string(vtree_path)?;
        let vtree = serde_json::from_str(&string)?;
        Ok(vtree)
    }

    pub fn get_commit(&self, commit_hash: &String) -> Result<Commit, Box<Error>> {
        let commit_path = match &self.backend {
            Backend::Local(backend) => format!(
                "{}{}/versions/{}.json",
                backend.path, self.path, commit_hash
            ),
            _ => panic!("Backend not implemented"),
        };
        let string = fs::read_to_string(commit_path)?;
        let commit = serde_json::from_str(&string)?;
        Ok(commit)
    }

    pub fn get_by_path(path: &String) -> Result<Dataset, Box<Error>> {
        debug!("Getting dataset by path: {}", path);
        let config_path = format!("./storage/{}/dataset.json", path);
        let string = fs::read_to_string(&config_path)?;
        let dataset: Dataset = serde_json::from_str(&string)?;
        Ok(dataset)
    }

    pub fn new(name: &String, path: &String, backend: Backend, description: &String) -> Dataset {
        info!("Creating new dataset with name {:?}", name);
        let dataset_path = match &backend {
            Backend::Local(backend) => format!("{}{}", backend.path, path),
            _ => panic!("Backend not implemented"),
        };

        debug!("Path is {}", dataset_path);

        // First create the folders. (does not do anything if the folder already exists)
        fs::create_dir_all(&dataset_path).expect("Could not create dataset directory..");
        fs::create_dir_all(format!("{}/data", &dataset_path))
            .expect("Could not create data directory..");
        fs::create_dir_all(format!("{}/versions", &dataset_path))
            .expect("Could not create versions directory..");
        fs::create_dir_all(format!("{}/runs", &dataset_path))
            .expect("Could not create runs directory..");

        // Write json files
        let dataset = Dataset {
            name: name.to_string(),
            path: path.to_string(),
            backend: backend,
            description: description.to_string(),
        };

        let dataset_string = serde_json::to_string_pretty(&dataset).unwrap();
        let mut dataset_file = File::create(format!("{}/dataset.json", dataset_path)).unwrap();
        dataset_file.write_all(&dataset_string.as_bytes()).unwrap();

        let initial_commit_hash = "initial_commit".to_string();
        let initial_branch_hash = "initial_branch".to_string();

        let commit = crate::commit::Commit {
            hash: initial_commit_hash.clone(),
            parent: None,
            branch: "master".to_string(),
            name: Some("root".to_string()),
            desc: "".to_string(),
            diff: vec![],
            deprecated: false,
        };

        let mut branches_map = HashMap::new();
        branches_map.insert(
            initial_branch_hash.clone(),
            Branch {
                hash: initial_branch_hash.clone(),
                name: "master".to_string(),
                head: initial_commit_hash.clone(),
            },
        );
        let mut vtree_map = HashMap::new();
        vtree_map.insert(
            initial_commit_hash,
            VersionTreeNode {
                name: "".to_string(),
                branch: initial_branch_hash.clone(),
                children: vec![],
                parent: None,
            },
        );
        let version_tree = VersionTree {
            branches: branches_map,
            tree: vtree_map,
        };

        let vtree_string = serde_json::to_string_pretty(&version_tree).unwrap();
        let mut vtree_file = File::create(format!("{}/vtree.json", dataset_path)).unwrap();
        vtree_file.write_all(&vtree_string.as_bytes()).unwrap();

        let commit_string = serde_json::to_string_pretty(&commit).unwrap();
        let mut commit_file =
            File::create(format!("{}/versions/{}.json", dataset_path, commit.hash)).unwrap();
        commit_file.write_all(&commit_string.as_bytes()).unwrap();

        dataset
    }
}
