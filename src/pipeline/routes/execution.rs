use crate::config;
use crate::dataset::models::DatasetConfig;
use crate::error::DaemonError;
use crate::pipeline::models::PipelineResult;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::pipeline::PipelineExecution;
use iterum_rust::utils;
use iterum_rust::vc::Dataset;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[get("/{dataset}/runs")]
async fn get_pipeline_executions(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    let dataset_path = path.into_inner();
    info!("Getting pipeline executions for {}", dataset_path);

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let pipeline_executions = dataset_config.backend.get_pipeline_executions(&dataset_config.name)?;

    Ok(HttpResponse::Ok().json(&pipeline_executions))
}

#[get("/runs/{pipeline_hash}")]
async fn get_pipeline_execution_without_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    let pipeline_hash = path.into_inner();
    info!("Getting pipeline execution with pipeline hash {}", pipeline_hash);

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let pipeline_execution = dataset_config
        .backend
        .get_pipeline_execution(&dataset_config.name, &pipeline_hash)?;

    Ok(HttpResponse::Ok().json(&pipeline_execution))
}

#[get("/{dataset}/runs/{pipeline_hash}")]
async fn get_pipeline_execution(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, pipeline_hash) = path.into_inner();
    info!(
        "Getting pipeline execution from {} for pipeline hash {}",
        dataset_path, pipeline_hash
    );

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let pipeline_execution = dataset_config
        .backend
        .get_pipeline_execution(&dataset_config.name, &pipeline_hash)?;

    Ok(HttpResponse::Ok().json(&pipeline_execution))
}

#[post("/{dataset}/runs")]
async fn create_pipeline_execution(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    pipeline_execution: web::Json<PipelineExecution>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new pipeline execution");

    let dataset_path = path.to_string();
    let pipeline_execution = pipeline_execution.into_inner();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    dataset_config
        .backend
        .store_pipeline_execution(&dataset_config, &pipeline_execution)?;

    Ok(HttpResponse::Ok().json(&pipeline_execution))
}

#[delete("/{dataset}/runs/{pipeline_hash}")]
async fn delete_pipeline_execution(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new pipeline execution");

    let (dataset_path, pipeline_hash) = path.into_inner();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    dataset_config
        .backend
        .remove_pipeline_execution(&dataset_config, &pipeline_hash)?;

    Ok(HttpResponse::Ok().finish())
}
