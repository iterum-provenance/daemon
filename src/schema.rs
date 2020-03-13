table! {
    branch (branch_hash) {
        branch_hash -> Text,
        name -> Text,
        head -> Text,
    }
}

table! {
    commit (hash) {
        hash -> Text,
        name -> Nullable<Text>,
        parent -> Nullable<Text>,
        branch -> Text,
        description -> Nullable<Text>,
        deprecated -> Nullable<Text>,
    }
}

table! {
    commit_diffs (hash, file, operation) {
        hash -> Text,
        file -> Text,
        operation -> Text,
        replacement -> Nullable<Text>,
    }
}

table! {
    committed_files (hash, file) {
        hash -> Text,
        file -> Text,
    }
}

table! {
    dataset (dataset_id) {
        dataset_id -> Integer,
        name -> Text,
        path -> Text,
        backend -> Text,
        description -> Text,
        head -> Nullable<Text>,
    }
}

table! {
    file (name) {
        name -> Text,
    }
}

joinable!(branch -> commit (head));
joinable!(commit_diffs -> commit (hash));
joinable!(committed_files -> commit (hash));
joinable!(committed_files -> file (file));
joinable!(dataset -> commit (head));

allow_tables_to_appear_in_same_query!(
    branch,
    commit,
    commit_diffs,
    committed_files,
    dataset,
    file,
);
