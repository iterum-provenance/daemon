use crate::config;
use crate::dataset::Dataset;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

use std::fs;

use std::path::Path;

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
        Err(error) => HttpResponse::NotFound().body(format!("{}", error)),
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
async fn get_dataset(config: web::Data<config::Config>, path: web::Path<String>) -> impl Responder {
    info!("Getting dataset with path {:?}", path);
    let dataset_path = path.to_string();
    match config.cache.get(dataset_path).unwrap() {
        Some(dataset_binary) => {
            let dataset: Dataset = dataset_binary.into();
            HttpResponse::Ok().json(dataset)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{dataset}/vtree")]
async fn get_vtree(config: web::Data<config::Config>, path: web::Path<String>) -> impl Responder {
    info!("Getting vtree from dataset with path {:?}", path);
    let dataset_path = path.to_string();
    match config.cache.get(dataset_path).unwrap() {
        Some(dataset_binary) => {
            let dataset: Dataset = dataset_binary.into();
            let vtree = dataset.backend.get_vtree(dataset.path);
            HttpResponse::Ok().json(vtree)
        }
        None => HttpResponse::NotFound().finish(),
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
            Err(_e) => HttpResponse::NotFound().finish(),
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
