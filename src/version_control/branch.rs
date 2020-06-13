use super::dataset::VCDataset;
use super::error::VersionControlError;
use crate::dataset::models::Branch;

impl VCDataset {
    pub fn add_branch(mut self, branch: &Branch) -> Result<VCDataset, VersionControlError> {
        // Check whether the commit does not already exist:
        if self.branches.contains_key(&branch.hash) {
            return Err(VersionControlError::BranchHashAlreadyExists);
        }
        // Check whether the head of the branch exists:
        if !self.version_tree.tree.contains_key(&branch.head) {
            return Err(VersionControlError::BranchHeadDoesNotExist);
        }

        self.branches
            .insert(branch.hash.to_string(), branch.clone());

        self.version_tree
            .branches
            .insert(branch.hash.to_string(), branch.name.to_string());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::tests::create_new_dataset;
    use iterum_rust::utils::create_random_hash;

    fn _create_dummy_branch(dataset: &VCDataset) -> Branch {
        let branch_hash = dataset.branches.iter().next().unwrap().0;
        let trunk = dataset.branches.get(branch_hash).unwrap();

        Branch {
            hash: create_random_hash(),
            name: "dummy".to_owned(),
            head: trunk.head.to_string(),
        }
    }
}
