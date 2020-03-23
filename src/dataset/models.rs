use crate::backend::Backend;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Add,
    Remove,
    Update,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diff {
    pub change_type: ChangeType,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub hash: String,
    pub parent: Option<String>,
    pub branch: String,
    pub name: Option<String>,
    pub desc: String,
    pub diff: Vec<Diff>,
    pub deprecated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub name: String,
    #[serde(flatten)]
    pub backend: Backend,
    pub description: String,
}

impl From<&Dataset> for sled::IVec {
    fn from(dataset: &Dataset) -> sled::IVec {
        debug!("Serializing struct {:?}", dataset);
        let string = serde_json::to_string(&dataset).expect("Serializing failed");
        string.into_bytes().into()
    }
}

impl From<sled::IVec> for Dataset {
    fn from(ivec: sled::IVec) -> Dataset {
        let string = String::from_utf8(ivec.to_vec()).expect("Converting bytes to string failed.");
        serde_json::from_str(&string).expect("Deserializing dataset failed")
    }
}

impl Dataset {}
