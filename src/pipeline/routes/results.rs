//! Contains routes with regards to results of a pipeline execution
use super::helpers::find_dataset_conf_for_pipeline_hash;
use crate::config;
use crate::dataset::models::DatasetConfig;
use crate::error::DaemonError;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::utils;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Creates a new results for a pipeline, and stores it on the storage backend
/// First stores the data in a temporary folder. Then redirects the data to the storage backend.
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

    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    // Store data in temporary folder
    let temp_path = format!("./.tmp/{}/", utils::create_random_hash());
    fs::create_dir_all(&temp_path).expect("Could not create temporary file directory.");
    let mut file_list: Vec<(String, String)> = Vec::new();
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
        debug!("Filename: {}", filename);

        let mut f = async_std::fs::File::create(&filepath).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
        file_list.push((filename.to_string(), filepath));
    }

    // Now move the files to the backend
    // Acquire write lock
    {
        let _datasets_ref = config.datasets.write().unwrap();
        dataset_config.store_pipeline_result_files(&file_list, &pipeline_hash, &temp_path.to_string())?;
        std::fs::remove_dir_all(&temp_path)?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Get specific result from a pipeline
#[get("/pipelines/{pipeline_hash}/results/{filename}")]
async fn get_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (pipeline_hash, file_name) = path.into_inner();
    info!("Getting pipeline result {}:{}", pipeline_hash, file_name);
    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let pipeline_result: Vec<u8> = dataset_config.get_pipeline_result(&pipeline_hash, &file_name)?;

    let file_path = Path::new(&file_name);
    let response = match file_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Something wrong with the file")
    {
        "jpg" => HttpResponse::Ok().content_type("image/jpeg").body(pipeline_result),
        _ => HttpResponse::Ok().body(pipeline_result),
    };

    Ok(response)
}

/// Get list of results for a pipeline
#[get("/pipelines/{pipeline_hash}/results")]
async fn get_pipeline_results(
    config: web::Data<config::Config>,
    path: web::Path<String>,
) -> Result<HttpResponse, DaemonError> {
    let pipeline_hash = path.into_inner();
    info!("Getting pipeline result {}", pipeline_hash);
    let dataset_config = match find_dataset_conf_for_pipeline_hash(&config.local_config, &pipeline_hash) {
        Some(conf) => conf,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let pipeline_result = dataset_config.get_pipeline_results(&pipeline_hash)?;

    Ok(HttpResponse::Ok().json(pipeline_result))
}
