use crate::backend::local::Local;
use crate::backend::Backend;
use crate::dataset::models::{
    Branch, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode,
};
use crate::utils::create_random_hash;
use crate::version_control::dataset::VCDataset;
use crate::version_control::error::VCErrorMessage;
use crate::version_control::error::VersionControlError;
use std::collections::HashMap;

pub fn create_new_dataset() -> VCDataset {
    let dataset_model = Dataset {
        name: "test_dataset".to_owned(),
        backend: Backend::Local({
            Local {
                path: "./storage/".to_owned(),
            }
        }),
        description: "niks".to_owned(),
    };
    VCDataset::new(dataset_model)
}
