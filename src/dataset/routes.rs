use crate::config;
use crate::dataset::{Branch, Dataset, VersionTree};
use crate::version_control;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};

use crate::backend::storable::Storable;
use crate::error::DaemonError;
use crate::utils;
use async_std::prelude::*;
use futures::StreamExt;
use serde_json::json;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;

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
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let file_data: Vec<u8> = dataset
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

    version_control::create_dataset(&dataset)?;
    config.cache.insert(&dataset.name, &dataset)?;

    Ok(HttpResponse::Ok().json(&dataset))
}

#[delete("/{dataset}")]
async fn delete_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Deleting dataset with path {:?}", path);
    let dataset_path = path.to_string();
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    dataset.backend.remove_dataset(&path)?;
    config.cache.remove(&dataset_path)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/{dataset}")]
async fn get_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Getting dataset with path {:?}", path);
    let dataset_path = path.to_string();
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    Ok(HttpResponse::Ok().json(dataset))
}

#[get("/{dataset}/vtree")]
async fn get_vtree(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Getting vtree from dataset with path {:?}", path);
    let dataset_path = path.to_string();
    let dataset: Dataset = config
        .cache
        .get(dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let vtree: VersionTree = dataset.backend.get_vtree(&dataset.name)?;
    Ok(HttpResponse::Ok().json(vtree))
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
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let branch: Branch = dataset.backend.get_branch(&dataset_path, &branch_hash)?;
    Ok(HttpResponse::Ok().json(branch))
}

#[post("/{dataset}/branch")]
async fn create_branch(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    branch: web::Json<Branch>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new branch with name {:?}", branch.name);
    let dataset_path = path.to_string();
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let branch = branch.into_inner();
    version_control::create_branch(&dataset, &branch)?;
    Ok(HttpResponse::Ok().json(&branch))
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
    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let commit = dataset.backend.get_commit(&dataset_path, &commit_hash)?;
    Ok(HttpResponse::Ok().json(commit))
}

#[post("/{dataset}/commit")]
async fn create_commit_with_data(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, DaemonError> {
    let dataset_path = path.to_string();
    info!("Posting commit with data to dataset {}", &dataset_path);

    // debug!("Acquiring lock for {}", &dataset_path);
    // config.state.get(&dataset_path)?;

    let dataset: Dataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    info!("Retrieved dataset");
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
        // fs::create_dir_all(&filepath).expect("Could not create temporary file directory.");
        let mut f = async_std::fs::File::create(filepath).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    debug!("Time to upload file \t{}ms", now.elapsed().as_millis());

    // Done uploading. Now parse the commit

    version_control::create_commit(&dataset, &temp_path)?;
    std::fs::remove_dir_all(&temp_path)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/reset_state")]
pub async fn reset_state(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Removing all state from the daemon.");
    for kv in config.cache.iter() {
        let (key, value) = kv?;
        let dataset: Dataset = value.into();
        debug!("Present in db: {:?}", dataset);
        dataset.backend.remove_dataset(&dataset.name).unwrap();
        config.cache.remove(key).unwrap();
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/")]
pub async fn get_datasets(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Retrieving all datasets.");
    let dataset_names: Vec<String> = config
        .cache
        .iter()
        .map(|kv| {
            let dataset: Dataset = kv.unwrap().1.into();
            dataset.name
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
