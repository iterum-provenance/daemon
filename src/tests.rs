use crate::backend::local::Local;
use crate::backend::Backend;
use crate::version_control::dataset::VCDataset;

pub fn create_new_dataset() -> VCDataset {
    VCDataset::new()
}
