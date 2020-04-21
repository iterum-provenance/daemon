use crate::backend::storable::Storable;
use crate::config;
use crate::pipeline::models::PipelineResult;
use crate::version_control;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};

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

#[post("/{dataset}/pipeline_result")]
async fn create_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    pipeline: web::Json<PipelineResult>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new pipeline with hash {:?}", pipeline);
    let dataset_path = path.to_string();
    let pipeline = pipeline.into_inner();

    let mut vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    vc_dataset = vc_dataset.add_pipeline_result(&pipeline)?;
    vc_dataset
        .dataset
        .backend
        .save_vcdataset(&dataset_path, &vc_dataset)?;
    config.cache.insert(&dataset_path, &vc_dataset)?;
    Ok(HttpResponse::Ok().json(&pipeline))
}

#[get("/{dataset}/pipeline_result/{pipeline_hash}")]
async fn get_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, pipeline_hash) = path.into_inner();
    info!(
        "Getting pipeline result {} from dataset {}",
        pipeline_hash, dataset_path
    );
    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();
    let pipeline_result = vc_dataset
        .pipeline_results
        .get(&pipeline_hash)
        .ok_or_else(|| DaemonError::NotFound)?;
    Ok(HttpResponse::Ok().json(pipeline_result))
}

#[post("/{dataset}/pipeline_result/{pipeline_hash}")]
async fn add_result(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
    mut payload: Multipart,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, pipeline_hash) = path.into_inner();
    info!(
        "Posting file to pipelineresult {} on dataset {}",
        pipeline_hash, dataset_path
    );

    let mut vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    // iterate over multipart stream
    let now = Instant::now();
    let temp_path = format!("./.tmp/{}/", utils::create_random_hash());
    fs::create_dir_all(&temp_path).expect("Could not create temporary file directory.");

    let mut file_list = Vec::new();

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

        let mut f = async_std::fs::File::create(&filepath).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
        file_list.push(filepath);
    }
    debug!("Time to upload file \t{}ms", now.elapsed().as_millis());

    // Done uploading. Now parse the commit

    // Check whether a branch file is present
    // let temp_branch_file = format!("{}/branch.json", temp_path);
    // if std::path::Path::new(&temp_branch_file).exists() {
    //     // Create the branch
    //     let branch_string: String = fs::read_to_string(temp_branch_file)?;
    //     let branch: Branch = serde_json::from_str(&branch_string)?;
    //     debug!("Creating branch with hash: {}", branch.hash);

    //     // Add branch to dataset.
    //     vc_dataset = vc_dataset.add_branch(&branch)?;
    // };
    let mut pipeline_result = vc_dataset
        .pipeline_results
        .get(&pipeline_hash)
        .ok_or_else(|| DaemonError::NotFound)?
        .clone();

    pipeline_result.files.extend(file_list);
    // Create commit
    // let temp_commit_file = format!("{}/commit", temp_path);
    // let commit_string: String = fs::read_to_string(temp_commit_file)?;
    // let commit: Commit = serde_json::from_str(&commit_string)?;

    // Add commit to the dataset

    vc_dataset = vc_dataset.set_pipeline_result(&pipeline_result)?;

    // Now move the files to the backend
    vc_dataset.dataset.backend.store_pipeline_result_files(
        &vc_dataset.dataset,
        &pipeline_result.hash,
        &temp_path.to_string(),
    )?;

    // Store changes to dataset to backend and caches.
    vc_dataset
        .dataset
        .backend
        .save_vcdataset(&dataset_path, &vc_dataset)?;
    config.cache.insert(dataset_path.to_string(), &vc_dataset)?;

    std::fs::remove_dir_all(&temp_path)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_pipeline_result);
    cfg.service(get_pipeline_result);
}
