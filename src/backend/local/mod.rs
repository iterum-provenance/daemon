//! Contains logic regarding communication with the local storage backend.

pub mod dataset;
pub mod pipeline;
use serde::{Deserialize, Serialize};

/// Local storage struct. Currently only has the `path` as a credential. For the current implementation this corresponds to where the Kubernetes PersistentVolume is mounted.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Local {
    pub path: String,
}
