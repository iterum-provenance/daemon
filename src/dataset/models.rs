use crate::backend::Backend;
use crate::commit;
use crate::commit::Commit;
use crate::dataset::error::DatasetError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

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
    pub branches: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub backend: Backend,
    pub description: String,
}

impl Dataset {
    pub fn read_from_path(path: &String) -> Result<Dataset, Box<dyn Error>> {
        let dataset_path = format!("{}/dataset.json", &path);
        let string: String = fs::read_to_string(&dataset_path)?;
        let item: Dataset = serde_json::from_str(&string)?;
        Ok(item)
    }

    fn check_if_commit_valid(&self, commit: &commit::Commit) -> bool {
        let version_tree = self.get_vtree().unwrap();
        match &commit.parent {
            Some(parent) => {
                debug!("The commit has a parent.");
                // If the parent is present in the tree, the commit can be added.
                if version_tree.tree.contains_key(parent)
                    && version_tree.branches.contains_key(&commit.branch)
                {
                    // But first check if the branch is also present in the dataset.
                    true
                } else {
                    false
                }
            }
            None => {
                debug!("The commit has no parent. Only the root node can exist in this state.");
                false
            }
        }
    }

    pub fn add_commit(&self, commit: &commit::Commit) -> Result<(), DatasetError> {
        let dataset_path = match &self.backend {
            Backend::Local(backend) => format!("{}{}", backend.path, self.path),
            _ => panic!("Backend not implemented"),
        };

        // Create commit file

        // Update version tree
        // First check if the parent of the commit is actually present. If not, the commit is invalid.
        if self.check_if_commit_valid(&commit) {
            let mut version_tree = self.get_vtree().unwrap();
            let commit_string = serde_json::to_string_pretty(&commit).unwrap();
            let mut commit_file =
                File::create(format!("{}/versions/{}.json", dataset_path, commit.hash)).unwrap();
            commit_file.write_all(&commit_string.as_bytes()).unwrap();
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
            Ok(())
        } else {
            Err(DatasetError::new(format!("Commit was not valid.")))
        }
    }

    pub fn get_vtree(&self) -> Result<VersionTree, Box<dyn Error>> {
        let vtree_path = match &self.backend {
            Backend::Local(backend) => format!("{}{}/vtree.json", backend.path, self.path),
            _ => panic!("Backend not implemented"),
        };
        let string = fs::read_to_string(vtree_path)?;
        let vtree = serde_json::from_str(&string)?;
        Ok(vtree)
    }

    pub fn get_branch(&self, branch_hash: &String) -> Result<Branch, Box<dyn Error>> {
        let branch_path = match &self.backend {
            Backend::Local(backend) => format!(
                "{}{}/branches/{}.json",
                backend.path, self.path, branch_hash
            ),
            _ => panic!("Backend not implemented"),
        };
        let string = fs::read_to_string(branch_path)?;
        let branch = serde_json::from_str(&string)?;
        Ok(branch)
    }

    pub fn get_commit(&self, commit_hash: &String) -> Result<Commit, Box<dyn Error>> {
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

    pub fn get_by_path(path: &String) -> Result<Dataset, Box<dyn Error>> {
        debug!("Getting dataset by path: {}", path);
        let config_path = format!("./storage/{}/dataset.json", path);
        let string = fs::read_to_string(&config_path)?;
        let dataset: Dataset = serde_json::from_str(&string)?;
        Ok(dataset)
    }

    pub fn get_path(&self) -> String {
        match &self.backend {
            Backend::Local(backend) => format!("{}{}", backend.path, self.path),
            _ => panic!("Backend not implemented"),
        }
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
        fs::create_dir_all(format!("{}/branches", &dataset_path))
            .expect("Could not create branches directory..");
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

        let branch = Branch {
            hash: initial_branch_hash.clone(),
            name: "master".to_string(),
            head: initial_commit_hash.clone(),
        };

        let branch_string = serde_json::to_string_pretty(&branch).unwrap();
        let mut branch_file = File::create(format!(
            "{}/branches/{}.json",
            dataset_path, initial_branch_hash
        ))
        .unwrap();
        branch_file.write_all(&branch_string.as_bytes()).unwrap();

        let mut branches_map = HashMap::new();
        branches_map.insert(initial_branch_hash.clone(), "master".to_string());
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
