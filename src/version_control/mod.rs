use crate::backend::storable::Storable;
use crate::dataset::{Branch, Commit, Dataset, Deprecated, Diff, VersionTree, VersionTreeNode};
use crate::error::{CommitError, DaemonError};
use crate::utils::create_random_hash;
use std::collections::HashMap;
use std::fs;

pub mod dataset;
pub mod error;

pub mod branch;
pub mod commit;

pub fn create_dataset(dataset: &Dataset) -> Result<(), DaemonError> {
    let mut tree: HashMap<String, VersionTreeNode> = HashMap::new();
    let root_commit_hash = create_random_hash();
    let master_branch_hash = create_random_hash();

    let root_commit = Commit {
        hash: root_commit_hash.to_string(),
        parent: None,
        branch: master_branch_hash.to_string(),
        name: "root".to_owned(),
        description: "".to_owned(),
        files: vec![],
        diff: Diff {
            added: vec![],
            updated: vec![],
            removed: vec![],
        },
        deprecated: Deprecated {
            value: false,
            reason: "".to_owned(),
        },
    };
    let vtree_root_node = VersionTreeNode {
        name: "root".to_owned(),
        branch: master_branch_hash.to_string(),
        children: vec![],
        parent: None,
    };
    tree.insert(root_commit_hash.to_string(), vtree_root_node);
    let master_branch = Branch {
        hash: master_branch_hash.to_string(),
        name: "master".to_owned(),
        head: root_commit_hash.to_string(),
    };

    let mut branches = HashMap::new();
    branches.insert(master_branch.hash.clone(), "master".to_owned());

    let vtree = VersionTree {
        tree: tree,
        branches: branches,
    };

    dataset.backend.create_dataset(&dataset)?;
    dataset.backend.set_vtree(&dataset.name, &vtree)?;
    dataset.backend.set_branch(&dataset.name, &master_branch)?;
    dataset.backend.create_commit(&dataset.name, &root_commit)?;
    Ok(())
}

pub fn create_commit(dataset: &Dataset, tmp_path: &String) -> Result<Commit, DaemonError> {
    let mut vtree = dataset.backend.get_vtree(&dataset.name)?.clone();

    let temp_commit_file = format!("{}/commit", tmp_path);
    let commit_string: String = fs::read_to_string(temp_commit_file)?;
    let commit: Commit = serde_json::from_str(&commit_string)?;

    // Check whether the commit does not already exist:
    if vtree.tree.contains_key(&commit.hash) {
        return Err(DaemonError::CommitError(CommitError::new(format!(
            "The commit hash already exists."
        ))));
    }

    // Check whether a branch file is present
    let temp_branch_file = format!("{}/branch", tmp_path);

    let mut branch = if std::path::Path::new(&temp_branch_file).exists() {
        debug!("Creating branch!");
        // Create the branch
        let branch_string: String = fs::read_to_string(temp_branch_file)?;
        let branch: Branch = serde_json::from_str(&branch_string)?;

        vtree
            .branches
            .insert(branch.hash.to_string(), branch.name.to_string());
        branch
    } else {
        dataset.backend.get_branch(&dataset.name, &commit.branch)?
    };

    let parent = match &commit.parent {
        Some(parent) => parent,
        None => {
            return Err(DaemonError::CommitError(CommitError::new(format!(
                "Commit has no parent. Only the root commit can exist in this state."
            ))))
        }
    };

    let mut parent_node = match vtree.tree.get(&parent.to_owned()) {
        Some(parent_node) => parent_node.clone(),
        None => {
            return Err(DaemonError::CommitError(CommitError::new(format!(
                "The parent commit does not exist in the version tree."
            ))))
        }
    };

    // Maybe also add a check that two commits in the same branch cannot have the same parent?
    // (This is basically the same as checking whether the head of the current branch is the
    //  same as the parent of the new commit.)
    if *parent != branch.head {
        return Err(DaemonError::CommitError(CommitError::new(format!(
            "The commit is not up to date with the head of the branch."
        ))));
    }

    branch.head = commit.hash.to_string();

    // First update the parent in the Vtree
    parent_node.children.push(commit.hash.to_owned());
    vtree.tree.insert(parent.to_string(), parent_node);

    // Create a new Vtree node, and add to the tree.
    let vtree_node = VersionTreeNode {
        name: "".to_owned(),
        branch: commit.branch.to_string(),
        children: vec![],
        parent: Some(parent.to_owned()),
    };
    vtree.tree.insert(commit.hash.to_string(), vtree_node);

    // Next operations should be a transaction in order to maintain a consistent state.
    // Now save the version tree
    dataset.backend.set_vtree(&dataset.name, &vtree)?;

    // Also store the commit itself.
    dataset.backend.create_commit(&dataset.name, &commit)?;

    // Store the branch if this was present
    dataset.backend.set_branch(&dataset.name, &branch)?;

    // Then store the files accordingly.
    dataset
        .backend
        .store_committed_files(&dataset, &commit, tmp_path.to_string())?;

    Ok(commit)
}

pub fn create_branch(dataset: &Dataset, branch: &Branch) -> Result<(), DaemonError> {
    let mut vtree = dataset.backend.get_vtree(&dataset.name)?.clone();
    if vtree.branches.contains_key(&branch.hash) {
        return Err(DaemonError::AlreadyExists);
    }
    if !vtree.tree.contains_key(&branch.head) {
        return Err(DaemonError::CommitError(CommitError::new(format!(
            "The branch HEAD does not exist in the version tree."
        ))));
    }
    vtree
        .branches
        .insert(branch.hash.clone(), branch.name.to_string());

    dataset.backend.set_vtree(&dataset.name, &vtree)?;
    dataset.backend.set_branch(&dataset.name, branch)?;
    Ok(())
}
