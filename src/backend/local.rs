use super::storable::Storable;
use crate::commit::{ChangeType, Commit};
use crate::dataset::Dataset;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Local {
    pub path: String,
}

impl Local {
    pub fn new(path: &String) -> Local {
        Local {
            path: path.to_string(),
        }
    }
}

impl Storable for Local {
    fn store_commit(&self, dataset: &Dataset, path: String) -> Result<(), std::io::Error> {
        debug!("Storing commit in backend.");
        debug!("Reading path {}.", path);

        let config_path = format!("{}commit.json", path);
        let commit_string = fs::read_to_string(config_path).unwrap();
        let commit: Commit = serde_json::from_str(&commit_string).unwrap();

        // // Create the new files wherever necessary
        for item in commit.diff {
            match item.change_type {
                ChangeType::Add => {
                    debug!("Adding files with names:");
                    for file in item.files {
                        let tmp_file_path = format!("{}{}", &path, file);
                        debug!("Pulling file from: {}", tmp_file_path);

                        let file_dir = format!("{}{}/data/{}", self.path, dataset.path, file);
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

        Ok(())
    }
}
