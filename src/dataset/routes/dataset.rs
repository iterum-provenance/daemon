use crate::config;
use crate::dataset::{DatasetConfig};
use crate::version_control;

use actix_web::{delete, get, post, web, HttpResponse};

use crate::backend::storable::Storable;
use crate::error::DaemonError;
use async_std::prelude::*;
use futures::StreamExt;

use serde_json::json;





use version_control::dataset::VCDataset;


#[post("/")]
pub async fn create_dataset(
    config: web::Data<config::Config>,
    dataset_config: web::Json<DatasetConfig>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new dataset with name {:?}", dataset_config.name);
    let dataset_config = dataset_config.into_inner();
    let dataset_path = &dataset_config.name;

    if config.local_config.contains_key(dataset_path).unwrap() {
        return Err(DaemonError::AlreadyExists);
    }
    config
        .local_config
        .insert(dataset_path.to_string(), &dataset_config)
        .unwrap();
    let vc_dataset = VCDataset::new();
    dataset_config.backend.save_vcdataset(dataset_path, &vc_dataset)?;
    config
        .datasets
        .write()
        .unwrap()
        .insert(dataset_path.to_string(), vc_dataset);
    Ok(HttpResponse::Ok().json(dataset_config))
}

#[get("/")]
pub async fn get_datasets(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Retrieving all datasets.");

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
    dataset_config.remove_vcdataset().unwrap();
    config.local_config.remove(&dataset_path)?;

    Ok(HttpResponse::Ok().finish())
}
