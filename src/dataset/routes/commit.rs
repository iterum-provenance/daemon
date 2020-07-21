use crate::config;
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::utils;
use iterum_rust::vc::{error::VersionControlError, Branch, Commit, Dataset};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[post("/{dataset}/commit")]
async fn create_commit_with_data(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, DaemonError> {
    let dataset_path = path.to_string();
    info!("Posting commit with data to dataset {}", &dataset_path);

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let mut vc_dataset: Dataset = config
        .datasets
        .read()
        .unwrap()
        .get(&dataset_path)
        .ok_or_else(|| DaemonError::NotFound)?
        .clone();

    // iterate over multipart stream
    let now = Instant::now();
    let temp_path = format!("./.tmp/{}/", utils::create_random_hash());
    fs::create_dir_all(&temp_path).expect("Could not create temporary file directory.");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disp = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_disp
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filepath = format!("{}{}", &temp_path, &filename);
        debug!("Saving file to {}", filepath);
        let parent_path = std::path::Path::new(&filepath).parent().unwrap();
        if !parent_path.exists() {
            fs::create_dir_all(&parent_path).expect("Could not create temporary file directory.");
        }

        let mut f = async_std::fs::File::create(filepath).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }

    debug!("Time to upload file \t{}ms", now.elapsed().as_millis());

    // Done uploading. Now parse the commit

    // Check whether a branch file is present
    let temp_branch_file = format!("{}/branch.json", temp_path);
    if std::path::Path::new(&temp_branch_file).exists() {
        // Create the branch
        let branch_string: String = fs::read_to_string(temp_branch_file)?;
        let branch: Branch = serde_json::from_str(&branch_string)?;
        debug!("Creating branch with hash: {}", branch.hash);

        // Add branch to dataset.
        vc_dataset = vc_dataset.add_branch(&branch)?;
    };

    // Create commit
    let temp_commit_file = format!("{}/commit", temp_path);
    let commit_string: String = fs::read_to_string(temp_commit_file)?;
    let commit: Commit = serde_json::from_str(&commit_string)?;

    // Add commit to the dataset
    vc_dataset = vc_dataset.add_commit(&commit)?;
    debug!("Adding commit with hash {} to dataset.", commit.hash);

    // Now move the files to the backend

    // Acquire write lock
    {
        let mut datasets_ref = config.datasets.write().unwrap();
        dataset_config.store_committed_files(&commit, temp_path.to_string())?;
        dataset_config.save_dataset(&vc_dataset)?;
        datasets_ref.insert(dataset_path.to_string(), vc_dataset);
        std::fs::remove_dir_all(&temp_path)?;
    }

    let datasets = config.datasets.read().unwrap();
    let vc_dataset = datasets.get(&dataset_path).unwrap();

    let mut response_map: HashMap<String, serde_json::Value> = HashMap::new();
    response_map.insert("vtree".to_owned(), serde_json::to_value(&vc_dataset.version_tree)?);
    let branch = vc_dataset.branches.get(&commit.branch).unwrap();
    response_map.insert("branch".to_owned(), serde_json::to_value(&branch)?);

    Ok(HttpResponse::Ok().json(response_map))
}

#[get("/{dataset}/commit/{commit}")]
async fn get_commit(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, commit_hash) = path.into_inner();
    info!(
        "Getting commit with hash \"{}\" from dataset with path {}",
        commit_hash, dataset_path
    );

    let datasets = config.datasets.read().unwrap();
    let vc_dataset = datasets.get(&dataset_path).ok_or_else(|| DaemonError::NotFound)?;

    let commit = vc_dataset
        .commits
        .get(&commit_hash)
        .ok_or_else(|| VersionControlError::CommitNotFound)?;
    Ok(HttpResponse::Ok().json(commit))
}
