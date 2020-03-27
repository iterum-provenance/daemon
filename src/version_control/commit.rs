use super::dataset::VCDataset;
use super::error::{VCErrorMessage, VersionControlError};
use crate::dataset::models::{Commit, VersionTreeNode};

impl VCDataset {
    pub fn add_commit(mut self, commit: &Commit) -> Result<VCDataset, VersionControlError> {
        // Check whether the commit does not already exist:
        if self.version_tree.tree.contains_key(&commit.hash) {
            return Err(VersionControlError::CommitHashAlreadyExists);
        }

        // Check whether commit has a parent or not
        let parent_hash = match &commit.parent {
            Some(parent) => parent,
            None => {
                return Err(VersionControlError::CommitIncomplete(VCErrorMessage::new(
                    "The parent commit does not exist in the version tree.".to_string(),
                )))
            }
        };

        // Check if the parent exists
        let mut parent_node = match &self.version_tree.tree.get(&parent_hash.to_owned()) {
            Some(parent_node) => (*parent_node).clone(),
            None => return Err(VersionControlError::ParentCommitNotFound),
        };

        // Check if the branch exists
        let mut branch = match &self.branches.get(&commit.branch) {
            Some(branch) => (*branch).clone(),
            None => return Err(VersionControlError::BranchNotFound),
        };

        // Maybe also add a check that two commits in the same branch cannot have the same parent?
        // (This is basically the same as checking whether the head of the current branch is the
        //  same as the parent of the new commit.)
        if *parent_hash != branch.head {
            return Err(VersionControlError::ParentCommitIsNotBranchHead);
        }

        // All checks were OK, now update the data structures.
        // Update head of branch
        branch.head = commit.hash.to_string();
        self.branches.insert(branch.hash.to_string(), branch);

        // Update the parent in the Vtree
        parent_node.children.push(commit.hash.to_string());
        self.version_tree
            .tree
            .insert(parent_hash.to_string(), parent_node);

        // Create a new Vtree node, and add to the tree.
        let vtree_node = VersionTreeNode {
            name: commit.name.to_string(),
            branch: commit.branch.to_string(),
            children: vec![],
            parent: Some(parent_hash.to_string()),
        };
        self.version_tree
            .tree
            .insert(commit.hash.to_string(), vtree_node);

        // Insert commit in commit map
        self.commits
            .insert(commit.hash.to_string(), (*commit).clone());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::models::{Deprecated, Diff};
    use crate::tests::create_new_dataset;
    use crate::utils::create_random_hash;

    fn create_dummy_commit(dataset: &VCDataset) -> Commit {
        let branch_hash = dataset.branches.iter().next().unwrap().0;
        let branch = dataset.branches.get(branch_hash).unwrap();
        let mut commit = Commit {
            hash: "".to_owned(),
            name: "".to_owned(),
            parent: Some("".to_owned()),
            branch: "".to_owned(),
            description: "".to_owned(),
            deprecated: Deprecated {
                value: false,
                reason: "".to_owned(),
            },
            diff: Diff {
                added: vec![],
                updated: vec![],
                removed: vec![],
            },
            files: vec![],
        };

        commit.branch = branch_hash.to_string();
        commit.hash = create_random_hash();
        commit.parent = Some(branch.head.to_string());

        commit
    }

    #[test]
    fn add_normal_commit() {
        let dataset = create_new_dataset();
        let commit = create_dummy_commit(&dataset);

        let updated_dataset = dataset.add_commit(&commit).unwrap();

        // Check if the commit is added to the map
        assert!(
            updated_dataset.commits.contains_key(&commit.hash),
            "The commit was not added to the commit map."
        );

        // Check if the commit is added to the Vtree
        assert!(
            updated_dataset.version_tree.tree.contains_key(&commit.hash),
            "The commit was not added to the version tree."
        );

        // Check if the head of the branch is updated
        assert!(
            updated_dataset.branches.get(&commit.branch).unwrap().head == commit.hash,
            "The head of the branch was not updated properly."
        );
    }

    #[test]
    fn add_same_commit_twice() {
        let dataset = create_new_dataset();
        let commit = create_dummy_commit(&dataset);

        let dataset_added_once = dataset.add_commit(&commit).unwrap();
        let error = dataset_added_once.add_commit(&commit).err().unwrap();

        match error {
            VersionControlError::CommitHashAlreadyExists => (),
            _ => panic!("Error returned was not the right one."),
        }
    }

    #[test]
    fn add_commit_without_parent_commit() {
        let dataset = create_new_dataset();
        let mut commit = create_dummy_commit(&dataset);

        commit.parent = None;

        let error = dataset.add_commit(&commit).err().unwrap();
        match error {
            VersionControlError::CommitIncomplete(_) => (),
            _ => panic!("Error returned was not the right one."),
        }
    }

    #[test]
    fn add_commit_parent_is_not_head_of_branch() {
        let dataset = create_new_dataset();

        let first_commit = create_dummy_commit(&dataset);
        let second_commit = create_dummy_commit(&dataset);

        // Now commit the first
        let dataset_1commit = dataset.add_commit(&first_commit).unwrap();

        // Now try to commit the second. Should fail because the parent of the commit is not up to date.
        let error = dataset_1commit.add_commit(&second_commit).err().unwrap();
        match error {
            VersionControlError::ParentCommitIsNotBranchHead => (),
            _ => panic!("Error returned was not the right one."),
        }
    }

    #[test]
    fn add_commit_without_existing_branch() {
        let dataset = create_new_dataset();
        let mut commit = create_dummy_commit(&dataset);

        commit.branch = create_random_hash();

        let error = dataset.add_commit(&commit).err().unwrap();
        match error {
            VersionControlError::BranchNotFound => (),
            _ => panic!("Error returned was not the right one."),
        }
    }
}
