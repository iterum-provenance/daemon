use crate::backend::local::Local;
use crate::backend::Backend;
use crate::dataset::models::Dataset;
use crate::version_control::dataset::VCDataset;

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
    VCDataset::new(&dataset_model)
}
