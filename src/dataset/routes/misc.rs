//! Routes related to managing branches of a dataset

use crate::config;
use crate::dataset::DatasetConfig;
use crate::error::DaemonError;
use actix_web::{get, post, web, HttpResponse};
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;

/// Retrieves a file from a dataset. Used by both the CLI and the Fragmenter to retrieve files.
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

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    // Perhaps add a check to see if the file exists in the dataset?
    let file_data: Vec<u8> = dataset_config.get_file(&commit_hash, &filename)?;
    let file_path = Path::new(&filename);
    let response = match file_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Something wrong with the file")
    {
        "jpg" => HttpResponse::Ok().content_type("image/jpeg").body(file_data),
        _ => HttpResponse::Ok().body(file_data),
    };
    Ok(response)
}

/// Removes all of the datasets known to the daemon, and also clear the local kv-store.
#[post("/reset_state")]
pub async fn reset_state(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    debug!("Removing all state from the daemon.");

    let mut datasets_ref = config.datasets.write().unwrap();
    config.local_config.iter().for_each(|kv| {
        let (ivec_name, ivec_dataset) = kv.unwrap();
        let dataset_config: DatasetConfig = ivec_dataset.into();
        dataset_config.remove_dataset().unwrap();
        let name: String = String::from_utf8(ivec_name.to_vec()).expect("Converting bytes to string failed.");
        datasets_ref.remove(&name);
    });
    config.local_config.clear().unwrap();

    Ok(HttpResponse::Ok().finish())
}

/// Retrieve the version tree for the dataset.
#[get("/{dataset}/vtree")]
async fn get_vtree(config: web::Data<config::Config>, path: web::Path<String>) -> Result<HttpResponse, DaemonError> {
    info!("Getting vtree from dataset with path {:?}", path);
    let dataset_path = path.to_string();

    let datasets = config.datasets.read().unwrap();
    let vc_dataset = datasets.get(&dataset_path).ok_or_else(|| DaemonError::NotFound)?;

    Ok(HttpResponse::Ok().json(&vc_dataset.version_tree))
}
