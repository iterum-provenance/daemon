pub mod dataset;
pub mod pipeline;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Local {
    pub path: String,
}
