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

#[post("/{dataset}/branch")]
async fn create_branch(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    branch: web::Json<Branch>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new branch with name {:?}", branch.name);
    let dataset_path = path.to_string();
    let branch = branch.into_inner();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let mut vc_dataset: VCDataset = config
        .datasets
        .read()
        .unwrap()
        .get(&dataset_path)
        .ok_or_else(|| DaemonError::NotFound)?
        .clone();

    vc_dataset = vc_dataset.add_branch(&branch)?;

    {
        let mut datasets_ref = config.datasets.write().unwrap();
        dataset_config.save_vcdataset(&vc_dataset)?;
        datasets_ref.insert(dataset_path.to_string(), vc_dataset);
    }

    Ok(HttpResponse::Ok().json(&branch))
}

#[get("/{dataset}/branch/{branch_hash}")]
async fn get_branch(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, branch_hash) = path.into_inner();
    info!("Getting branch {} from dataset {}", branch_hash, dataset_path);

    let datasets = config.datasets.read().unwrap();
    let vc_dataset = datasets.get(&dataset_path).ok_or_else(|| DaemonError::NotFound)?;

    let branch = vc_dataset
        .branches
        .get(&branch_hash)
        .ok_or_else(|| VersionControlError::BranchNotFound)?;
    Ok(HttpResponse::Ok().json(branch))
}