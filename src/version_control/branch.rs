use super::dataset::VCDataset;
use super::error::{VCErrorMessage, VersionControlError};
use crate::dataset::models::{Branch, Commit, Dataset, Deprecated, Diff, VersionTreeNode};

impl VCDataset {
    pub fn add_branch(mut self, branch: &Branch) -> Result<VCDataset, VersionControlError> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::create_new_dataset;
    use crate::utils::create_random_hash;

    fn create_dummy_branch(dataset: &VCDataset) -> Branch {
        let branch_hash = dataset.branches.iter().next().unwrap().0;
        let trunk = dataset.branches.get(branch_hash).unwrap();

        let branch = Branch {
            hash: create_random_hash(),
            name: "dummy".to_owned(),
            head: trunk.head.to_string(),
        };

        branch
    }
}
