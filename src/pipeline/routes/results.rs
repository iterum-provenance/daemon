use crate::config;
use crate::pipeline::models::PipelineResult;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};
use iterum_rust::vc::Dataset;

use crate::dataset::models::DatasetConfig;
use crate::error::DaemonError;
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::utils;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[post("/{dataset}/pipeline_result")]
async fn create_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    pipeline: web::Json<PipelineResult>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new pipeline with hash {:?}", pipeline);
    let dataset_path = path.to_string();
    let pipeline = pipeline.into_inner();

    let _dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    let _vc_dataset: Dataset = config
        .datasets
        .read()
        .unwrap()
        .get(&dataset_path)
        .ok_or_else(|| DaemonError::NotFound)?
        .clone();
    Ok(HttpResponse::Ok().json(&pipeline))
}

// #[get("/{dataset}/pipelines")]
// async fn get_pipelines_for_dataset(
//     config: web::Data<config::Config>,
//     path: web::Path<String>,
// ) -> Result<HttpResponse, DaemonError> {
//     let dataset_path = path.into_inner();
//     info!(
//         "Getting pipelines for dataset {}
//     ",
//         dataset_path
//     );
//     let vc_dataset: Dataset = config
//         .cache
//         .get(&dataset_path)?
//         .ok_or_else(|| DaemonError::NotFound)?
//         .into();

//     // let pipeline_result = vc_dataset
//     //     .dataset
//     //     .backend
//     //     .get_pipeline_results(&dataset_path, &pipeline_hash)?;

//     Ok(HttpResponse::Ok().json(pipeline_result))
// }

#[get("/{dataset}/pipeline_result/{pipeline_hash}/{filename}")]
async fn get_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse, DaemonError> {
    let (dataset_path, pipeline_hash, file_name) = path.into_inner();
    info!(
        "Getting pipeline result {}:{} from dataset {}",
        pipeline_hash, file_name, dataset_path
    );
    let dataset_config: DatasetConfig = config
        .local_config
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

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

// #[get("/{dataset}/pipeline_result/{pipeline_hash}")]
// async fn get_pipeline_results(
//     config: web::Data<config::Config>,
//     path: web::Path<(String, String)>,
// ) -> Result<HttpResponse, DaemonError> {
//     let (dataset_path, pipeline_hash) = path.into_inner();
//     info!(
//         "Getting pipeline result {} from dataset {}",
//         pipeline_hash, dataset_path
//     );
//     let dataset_config: DatasetConfig = config
//         .local_config
//         .get(&dataset_path)?
//         .ok_or_else(|| DaemonError::NotFound)?
//         .into();

//     let pipeline_result = dataset_config.get_pipeline_results(&pipeline_hash)?;

//     Ok(HttpResponse::Ok().json(pipeline_result))
// }

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

    // iterate over multipart stream
    let now = Instant::now();
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
        // let parent_path = std::path::Path::new(&filepath).parent().unwrap();
        // if !parent_path.exists() {
        //     fs::create_dir_all(&parent_path).expect("Could not create temporary file directory.");
        // }

        let mut f = async_std::fs::File::create(&filepath).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
        file_list.push((filename.to_string(), filepath));
    }
    debug!("Time to upload file \t{}ms", now.elapsed().as_millis());

    // Now move the files to the backend
    // Acquire write lock
    {
        let _datasets_ref = config.datasets.write().unwrap();
        dataset_config.store_pipeline_result_files(&file_list, &pipeline_hash, &temp_path.to_string())?;
        std::fs::remove_dir_all(&temp_path)?;
    }

    Ok(HttpResponse::Ok().finish())
}
