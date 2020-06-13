use crate::config;
use crate::dataset::{Branch, Commit, DatasetConfig};
use crate::version_control;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};

use crate::backend::storable::Storable;
use crate::error::DaemonError;
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::utils;
use serde_json::json;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;
use version_control::dataset::VCDataset;
use version_control::error::VersionControlError;

#[get("/{dataset}/file/{file}/{commit}")]
async fn get_file(
    config: web::Data<config::Config>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, filename, commit_hash) = path.into_inner();
    info!(
        "Getting file {} from commit {} from dataset {}",
        filename, commit_hash, dataset_path
    );
    let now = Instant::now();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    // let datasets = config.datasets.read().unwrap();
    // let vc_dataset = datasets.get(&dataset_path).ok_or_else(|| DaemonError::NotFound)?;
    // Perhaps add a check to see if the file exists in the dataset?
    info!("{}:Retrieving dataset takes {}", filename, now.elapsed().as_millis());

    let now = Instant::now();
    let file_data: Vec<u8> = dataset_config.get_file(&commit_hash, &filename)?;
    info!("{}:Retrieving file takes {}", filename, now.elapsed().as_millis());

    let file_path = Path::new(&filename);
    let now = Instant::now();
    let response = match file_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Something wrong with the file")
    {
        "jpg" => HttpResponse::Ok().content_type("image/jpeg").body(file_data),
        _ => HttpResponse::Ok().body(file_data),
    };
    info!("{}:Constructing response takes {}", filename, now.elapsed().as_millis());
    Ok(response)
}

#[post("/reset_state")]
pub async fn reset_state(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Removing all state from the daemon.");

    let mut datasets_ref = config.datasets.write().unwrap();
    config.local_config.iter().for_each(|kv| {
        let (ivec_name, ivec_dataset) = kv.unwrap();
        let dataset_config: DatasetConfig = ivec_dataset.into();
        dataset_config.remove_vcdataset().unwrap();
        let name: String = String::from_utf8(ivec_name.to_vec()).expect("Converting bytes to string failed.");
        datasets_ref.remove(&name);
    });
    config.local_config.clear().unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("/{dataset}/vtree")]
async fn get_vtree(config: web::Data<config::Config>, path: web::Path<String>) -> Result<HttpResponse, DaemonError> {
    info!("Getting vtree from dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let datasets = config.datasets.read().unwrap();
    let vc_dataset = datasets.get(&dataset_path).ok_or_else(|| DaemonError::NotFound)?;

    Ok(HttpResponse::Ok().json(&vc_dataset.version_tree))
}
