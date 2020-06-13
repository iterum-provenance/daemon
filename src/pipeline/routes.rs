use crate::backend::storable::Storable;
use crate::config;
use crate::pipeline::models::PipelineResult;
use crate::version_control;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};

use crate::error::DaemonError;
use async_std::prelude::*;
use futures::StreamExt;
use iterum_rust::utils;
use std::fs;
use std::time::Instant;
use version_control::dataset::VCDataset;

#[post("/{dataset}/pipeline_result")]
async fn create_pipeline_result(
    config: web::Data<config::Config>,
    path: web::Path<String>,
    pipeline: web::Json<PipelineResult>,
) -> Result<HttpResponse, DaemonError> {
    info!("Creating new pipeline with hash {:?}", pipeline);
    let dataset_path = path.to_string();
    let pipeline = pipeline.into_inner();

    let _vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| DaemonError::NotFound)?
        .into();

    // vc_dataset = vc_dataset.add_pipeline_result(&pipeline)?;
    // vc_dataset
    //     .dataset
    //     .backend
    //     .save_vcdataset(&dataset_path, &vc_dataset)?;
    // config.cache.insert(&dataset_path, &vc_dataset)?;
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
//     let vc_dataset: VCDataset = config
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
        .dataset
        .backend
        .get_pipeline_results(&dataset_path, &pipeline_hash)?;

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

    let vc_dataset: VCDataset = config
        .cache
        .get(&dataset_path)?
        .ok_or_else(|| {
            error!("Could not find dataset..");
            DaemonError::NotFound
        })?
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
    vc_dataset.dataset.backend.store_pipeline_result_files(
        &vc_dataset.dataset,
        &file_list,
        &pipeline_hash,
        &temp_path.to_string(),
    )?;

    std::fs::remove_dir_all(&temp_path)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_pipeline_result);
    cfg.service(get_pipeline_result);
    cfg.service(add_result);
}
