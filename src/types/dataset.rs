use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Backend {
    LocalBackend(LocalBackend),
    AmazonS3Backend,
    GoogleCloudBackend
}

#[derive(Serialize, Deserialize)]
pub struct LocalBackend {
    pub path: String
}

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub backend: Backend,
    pub description: String
}


