use crate::backend::storable::Storable;
use crate::backend::Backend;
use crate::commit::ChangeType;
use crate::commit::Commit;
use crate::config;
use crate::dataset::Dataset;
use actix_multipart::Multipart;
use actix_web::{delete, error, get, post, web, HttpResponse, Responder};
use actix_web::{Error, Result};
use async_std::prelude::*;
use bytes::Bytes;
use futures::StreamExt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::time::Instant;

use std::path::Path;

// use async_std::prelude::*;

#[get("/{dataset}/file/{commit}/{file}")]
async fn get_file(
    _config: web::Data<config::Config>,
    info: web::Path<(String, String, String)>,
) -> impl Responder {
    info!("Retrieving file {} from {}:{}", info.2, info.0, info.1);

    let dataset = Dataset::get_by_path(&info.0).unwrap();
    let file_path = format!("{}/data/{}/{}", dataset.get_path(), &info.2, &info.1);
    debug!("Reading path {}", file_path);
    match fs::read(&file_path) {
        Ok(contents) => {
            let response = HttpResponse::Ok().content_type("image/jpeg").body(contents);
            response
        }
        Err(error) => HttpResponse::NotFound().body("Could not find the file"),
    }
}

#[post("/")]
async fn create_dataset(
    _config: web::Data<config::Config>,
    dataset: web::Json<Dataset>,
) -> impl Responder {
    info!("Creating new dataset with name {:?}", dataset.name);
    let dataset = dataset.into_inner();

    let constructed_dataset = Dataset::new(
        &dataset.name,
        &dataset.path,
        dataset.backend,
        &dataset.description,
    );

    HttpResponse::Ok().json(constructed_dataset)
}

#[delete("/{dataset}")]
async fn delete_dataset(
    _config: web::Data<config::Config>,
    path: web::Path<String>,
) -> impl Responder {
    info!("Deleting dataset with path {:?}", path);

    let dataset_path = format!("./storage/{}", path);
    if Path::new(&dataset_path).exists() {
        fs::remove_dir_all(&dataset_path).unwrap();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/{dataset}")]
async fn get_dataset(
    _config: web::Data<config::Config>,
    path: web::Path<String>,
) -> impl Responder {
    info!("Getting dataset with path {:?}", path);

    let dataset_path = format!("./storage/{}/dataset.json", path);
    if Path::new(&dataset_path).exists() {
        let string = fs::read_to_string(&dataset_path).unwrap();
        let object: Dataset = serde_json::from_str(&string).unwrap();
        HttpResponse::Ok().json(object)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/{dataset}/vtree")]
async fn get_vtree(_config: web::Data<config::Config>, path: web::Path<String>) -> impl Responder {
    info!("Getting vtree from dataset with path {:?}", path);
    match Dataset::get_by_path(&path) {
        Ok(dataset) => HttpResponse::Ok().json(dataset.get_vtree().unwrap()),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[get("/{dataset}/branch/{branch_hash}")]
async fn get_branch(
    _config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    info!("Getting branch from dataset with path {:?}", path.0);
    match Dataset::get_by_path(&path.0) {
        Ok(dataset) => match dataset.get_branch(&path.1) {
            Ok(commit) => HttpResponse::Ok().json(commit),
            Err(e) => HttpResponse::NotFound().finish(),
        },
        _ => HttpResponse::NotFound().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_dataset);
    cfg.service(get_dataset);
    cfg.service(create_dataset);
    cfg.service(get_vtree);
    cfg.service(get_branch);
    cfg.service(get_file);
}
