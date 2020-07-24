//! Contains the routes related to a PipelineExecution, which is a specific instance of a PipelineRun, with corresponding lineage info, status and results.
use super::helpers::{find_all_pipelines, find_dataset_conf_for_pipeline_hash};
use crate::config;
use crate::dataset::models::DatasetConfig;
use crate::error::DaemonError;
use actix_web::{delete, get, post, web, HttpResponse};
use iterum_rust::pipeline::PipelineExecution;

/// Retrieve the different pipeline executions for a dataset
#[get("/{dataset}/pipelines")]
async fn get_dataset_pipeline_executions(
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

/// Retrieve the different pipeline executions for all datasets
#[get("/pipelines")]
async fn get_pipeline_executions(config: web::Data<config::Config>) -> Result<HttpResponse, DaemonError> {
    info!("Getting pipeline executions");
    let pipeline_executions = find_all_pipelines(&config.local_config);
    Ok(HttpResponse::Ok().json(&pipeline_executions))
}

/// Retrieve a specific pipeline execution, without knowing the dataset beforehand
#[get("/pipelines/{pipeline_hash}")]
async fn get_pipeline_execution_without_dataset(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    let pipeline_hash = path.into_inner();
    info!("Getting pipeline execution with pipeline hash {}", pipeline_hash);

    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let pipeline_execution = dataset_config
        .backend
        .get_pipeline_execution(&dataset_config.name, &pipeline_hash)?;

    Ok(HttpResponse::Ok().json(&pipeline_execution))
}

/// Retrieve a specific pipeline execution
#[get("/{dataset}/pipelines/{pipeline_hash}")]
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

/// Create a new pipeline execution on a dataset
#[post("/{dataset}/pipelines")]
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

/// Delete a specific pipeline execution
#[delete("/pipelines/{pipeline_hash}")]
async fn delete_pipeline_execution(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Deleting new pipeline execution");

    let pipeline_hash = path.into_inner();

    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    dataset_config
        .backend
        .remove_pipeline_execution(&dataset_config, &pipeline_hash)?;

    Ok(HttpResponse::Ok().finish())
}
