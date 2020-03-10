use serde::{Deserialize, Serialize};

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
    pub parent: String,
    pub branch: String,
    pub name: String,
    pub desc: String,
    pub diff: Vec<Diff>,
    pub deprecated: bool,
}
