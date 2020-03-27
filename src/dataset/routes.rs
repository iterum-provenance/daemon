use crate::config;
use crate::dataset::{Branch, Commit, Dataset};
use crate::version_control;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};

use crate::backend::storable::Storable;
use crate::error::DaemonError;
use crate::utils;
use async_std::prelude::*;
use futures::StreamExt;
use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;
use version_control::dataset::VCDataset;
use version_control::error::VersionControlError;

#[get("/{dataset}/file/{commit}/{file}")]
async fn get_file(
    config: web::Data<config::Config>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, commit_hash, filename) = path.into_inner();
    info!(
        "Getting file {} from commit {} from dataset {}",
        filename, commit_hash, dataset_path
    );
    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let file_data: Vec<u8> =
        vc_dataset
            .dataset
            .backend
            .get_file(&dataset_path, &commit_hash, &filename)?;
    let file_path = Path::new(&filename);
    let response = match file_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Something wrong with the file")
    {
        "jpg" => HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(file_data),
        _ => unimplemented!(),
    };
    Ok(response)
}

#[post("/")]
async fn create_dataset(
    config: web::Data<config::Config>,
    dataset: web::Json<Dataset>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new dataset with name {:?}", dataset.name);
    let dataset = dataset.into_inner();
    let dataset_path = &dataset.name;

    let vc_dataset = VCDataset::new(&dataset);
    if config.cache.contains_key(&dataset_path).unwrap() {
        return Err(DaemonError::AlreadyExists);
    }
    config.cache.insert(&dataset_path, &vc_dataset)?;
    dataset.backend.save_vcdataset(&dataset_path, &vc_dataset)?;
    Ok(HttpResponse::Ok().json(dataset))
}

#[delete("/{dataset}")]
async fn delete_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Deleting dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    vc_dataset.dataset.backend.remove_vcdataset(&dataset_path)?;
    config.cache.remove(&dataset_path)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/{dataset}/branch")]
async fn create_branch(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    branch: web::Json<Branch>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new branch with name {:?}", branch.name);
    let dataset_path = path.to_string();
    let branch = branch.into_inner();

    let mut vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    vc_dataset = vc_dataset.add_branch(&branch)?;
    vc_dataset
        .dataset
        .backend
        .save_vcdataset(&dataset_path, &vc_dataset)?;
    config.cache.insert(&dataset_path, &vc_dataset)?;
    Ok(HttpResponse::Ok().json(&branch))
}

#[post("/{dataset}/commit")]
async fn create_commit_with_data(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, DaemonError> {
    let dataset_path = path.to_string();
    info!("Posting commit with data to dataset {}", &dataset_path);

    let mut vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

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
    vc_dataset.dataset.backend.store_committed_files(
        &vc_dataset.dataset,
        &commit,
        temp_path.to_string(),
    )?;

    // Store changes to dataset to backend and caches.
    vc_dataset
        .dataset
        .backend
        .save_vcdataset(&dataset_path, &vc_dataset)?;
    config.cache.insert(dataset_path.to_string(), &vc_dataset)?;

    std::fs::remove_dir_all(&temp_path)?;

    let mut response_map: HashMap<String, serde_json::Value> = HashMap::new();
    response_map.insert(
        "vtree".to_owned(),
        serde_json::to_value(&vc_dataset.version_tree)?,
    );
    let branch = vc_dataset.branches.get(&commit.branch).unwrap();
    response_map.insert("branch".to_owned(), serde_json::to_value(&branch)?);

    Ok(HttpResponse::Ok().json(response_map))
}

#[post("/reset_state")]
pub async fn reset_state(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Removing all state from the daemon.");

    config.cache.iter().for_each(|kv| {
        let (ivec_name, ivec_dataset) = kv.unwrap();
        let vc_dataset: VCDataset = ivec_dataset.into();
        let name: String =
            String::from_utf8(ivec_name.to_vec()).expect("Converting bytes to string failed.");
        vc_dataset.dataset.backend.remove_vcdataset(&name).unwrap();
    });
    config.cache.clear().unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("/{dataset}")]
async fn get_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Getting dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    Ok(HttpResponse::Ok().json(&vc_dataset.dataset))
}

#[get("/{dataset}/vtree")]
async fn get_vtree(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Getting vtree from dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    Ok(HttpResponse::Ok().json(&vc_dataset.version_tree))
}

#[get("/{dataset}/branch/{branch_hash}")]
async fn get_branch(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, branch_hash) = path.into_inner();
    info!(
        "Getting branch {} from dataset {}",
        branch_hash, dataset_path
    );
    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let branch = vc_dataset
        .branches
        .get(&branch_hash)
        .ok_or_else(|| VersionControlError::BranchNotFound)?;
    Ok(HttpResponse::Ok().json(branch))
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
    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let commit = vc_dataset
        .commits
        .get(&commit_hash)
        .ok_or_else(|| VersionControlError::CommitNotFound)?;
    Ok(HttpResponse::Ok().json(commit))
}

#[get("/")]
pub async fn get_datasets(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Retrieving all datasets.");

    let dataset_names: Vec<String> = config
        .cache
        .iter()
        .map(|kv| {
            let (ivec_name, ivec_dataset) = kv.unwrap();
            let _vc_dataset: VCDataset = ivec_dataset.into();
            let name: String =
                String::from_utf8(ivec_name.to_vec()).expect("Converting bytes to string failed.");
            name
        })
        .collect();
    Ok(HttpResponse::Ok().json(json!(dataset_names)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_dataset);
    cfg.service(get_dataset);
    cfg.service(create_dataset);
    cfg.service(get_vtree);
    cfg.service(get_branch);
    cfg.service(create_branch);
    cfg.service(get_file);
    cfg.service(get_commit);
    cfg.service(create_commit_with_data);
    cfg.service(reset_state);
    cfg.service(get_datasets);
}
