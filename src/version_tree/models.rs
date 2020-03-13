use std::collections::HashMap;

pub struct Branch {
    pub hash: String,
    pub name: String,
    pub head: String,
}

pub struct VersionTreeNode {
    pub name: String,
    pub branch: String,
    pub children: Vec<String>,
    pub parent: String,
}

pub struct VersionTree {
    pub tree: HashMap<String, VersionTreeNode>,
    pub branches: HashMap<String, String>,
}
