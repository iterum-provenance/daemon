//! Routes related to managing branches of a dataset

use crate::config;
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use actix_web::{delete, get, post, web, HttpResponse};
use iterum_rust::vc;
use serde_json::json;
use vc::Dataset;

/// Create a new dataset based on a provided DatasetConfig
#[post("/")]
pub async fn create_dataset(
    config: web::Data<config::Config>,
    dataset_config: web::Json<DatasetConfig>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new dataset with name {:?}", dataset_config.name);
    let dataset_config = dataset_config.into_inner();
    let dataset_path = &dataset_config.name;

    // Check whether the dataset does not already exist
    if config.local_config.contains_key(dataset_path).unwrap() {
        return Err(DaemonError::AlreadyExists);
    }
    config
        .local_config
        .insert(dataset_path.to_string(), &dataset_config)
        .unwrap();
    let vc_dataset = Dataset::new();
    dataset_config.backend.save_dataset(dataset_path, &vc_dataset)?;
    config
        .datasets
        .write()
        .unwrap()
        .insert(dataset_path.to_string(), vc_dataset);
    Ok(HttpResponse::Ok().json(dataset_config))
}

/// Retrieve datasets known to the Daemon
#[get("/")]
pub async fn get_datasets(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Retrieving all datasets.");

    // Retrieve from the local kv-store.
    let dataset_names: Vec<String> = config
        .local_config
        .iter()
        .map(|kv| {
            let (ivec_name, _) = kv.unwrap();
            let name: String = String::from_utf8(ivec_name.to_vec()).expect("Converting bytes to string failed.");
            name
        })
        .collect();
    Ok(HttpResponse::Ok().json(json!(dataset_names)))
}

/// Retrieve datasets known to the Daemon
#[get("/{dataset}")]
async fn get_dataset(config: web::Data<config::Config>, path: web::Path<String>) -> Result<HttpResponse, DaemonError> {
    info!("Getting dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    Ok(HttpResponse::Ok().json(&dataset_config))
}

/// Delete dataset from the daemon. Also removes all data related to this dataset from the storage backend.
#[delete("/{dataset}")]
async fn delete_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Deleting dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    config.datasets.write().unwrap().remove(&dataset_path).unwrap();
    dataset_config.remove_dataset().unwrap();
    config.local_config.remove(&dataset_path)?;

    Ok(HttpResponse::Ok().finish())
}
