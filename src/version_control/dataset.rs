use crate::dataset::models::{Branch, Commit, Deprecated, Diff, VersionTree, VersionTreeNode};
// use crate::pipeline::models::PipelineResult;
use iterum_rust::utils::create_random_hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VCDataset {
    // pub dataset: Dataset,
    pub commits: HashMap<String, Commit>,
    pub branches: HashMap<String, Branch>,
    pub version_tree: VersionTree,
}

// impl From<&VCDataset> for sled::IVec {
//     fn from(dataset: &VCDataset) -> sled::IVec {
//         debug!("Serializing struct {:?}", dataset);
//         let string = serde_json::to_string(&dataset).expect("Serializing failed");
//         string.into_bytes().into()
//     }
// }

// impl From<sled::IVec> for VCDataset {
//     fn from(ivec: sled::IVec) -> VCDataset {
//         let string = String::from_utf8(ivec.to_vec()).expect("Converting bytes to string failed.");
//         serde_json::from_str(&string).expect("Deserializing dataset failed")
//     }
// }

// Dataset struct waar alle json files in zitten wat je ook op de disc opslaat.
// Dan kan je makkelijker integrity checks doen.
// Functies die een dataset krijgen + een aanpassing, en dan kan je zeggen of het mag of niet.
// Je kan dan een specifieke error terugsturen waarom iets niet mag.
// Verder kan je makkelijk unit tests schrijven voor de verschillende integrity checks.

// Je houd dan die dataset struct in de lokale cache. Je kan dan ook een RwLock daar opslaan.
// De data zou dan wel tot elke worker beschikbaar moeten zijn.

// Functies hebben dan de signature: Dataset + aanpassing -> Result<Aangepaste Dataset, Error>
// Vervolgens kan de caller de nieuwe dataset opslaan.

impl VCDataset {
    pub fn new() -> VCDataset {
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
        let version_tree = VersionTree { tree, branches };

        let mut commit_map: HashMap<String, Commit> = HashMap::new();
        commit_map.insert(root_commit_hash, root_commit);

        let mut branch_map: HashMap<String, Branch> = HashMap::new();
        branch_map.insert(master_branch_hash, master_branch);

        VCDataset {
            commits: commit_map,
            branches: branch_map,
            version_tree,
        }
    }
}
