//! Contains routes with regards to provenance tracking for pipelines
use super::helpers::find_dataset_conf_for_pipeline_hash;
use crate::config;
use crate::dataset::models::DatasetConfig;
use crate::error::DaemonError;
use actix_web::{get, post, web, HttpResponse};
use iterum_rust::provenance::FragmentLineage;

/// Creates a new FragmentLineage on the storage backend
#[post("/{dataset}/pipelines/{pipeline_hash}/lineage")]
async fn post_fragment_lineage(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
    fragment_lineage: web::Json<FragmentLineage>,
) -> Result<HttpResponse, DaemonError> {
    info!("Posting fragment lineage");

    let (dataset_path, pipeline_hash) = path.into_inner();
    let fragment_lineage = fragment_lineage.into_inner();

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    dataset_config
        .backend
        .store_pipeline_fragment_lineage(&dataset_config, &pipeline_hash, &fragment_lineage)?;

    Ok(HttpResponse::Ok().finish())
}

/// Retrieves a list of FragmentLineages from the storage backend
#[get("/pipelines/{pipeline_hash}/lineage")]
async fn get_fragment_lineages(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    info!("Retrieving fragment lineages");

    let pipeline_hash = path.into_inner();

    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let fragment_lineages = dataset_config
        .backend
        .get_pipeline_fragment_lineages(&dataset_config, &pipeline_hash)?;

    Ok(HttpResponse::Ok().json(fragment_lineages))
}

/// Retrieves a specific FragmentLineage from the storage backend
#[get("/pipelines/{pipeline_hash}/lineage/{fragment_id}")]
async fn get_fragment_lineage(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    info!("Retrieving fragment lineage");

    let (pipeline_hash, fragment_id) = path.into_inner();

    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let fragment_lineage =
        dataset_config
            .backend
            .get_pipeline_fragment_lineage(&dataset_config, &pipeline_hash, &fragment_id)?;

    Ok(HttpResponse::Ok().json(fragment_lineage))
}
