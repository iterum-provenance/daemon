use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Backend {
    LocalBackend { path: String },
    AmazonS3Backend,
    GoogleCloudBackend,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub backend: Backend,
    pub description: String,
}
