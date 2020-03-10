use crate::config;
use crate::dataset::{Backend, Dataset};
use actix_web::{delete, error, get, post, web, HttpResponse, Responder};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
// use async_std::prelude::*;

#[get("/{dataset}/{commit}/{file}")]
async fn get_file(
    _config: web::Data<config::Config>,
    info: web::Path<(String, String, String)>,
) -> impl Responder {
    info!("Retrieving file {} from {}:{}", info.2, info.0, info.1);

    HttpResponse::Ok()
}

#[post("/datasets/")]
async fn create_dataset(
    _config: web::Data<config::Config>,
    dataset: web::Json<Dataset>,
) -> impl Responder {
    info!("Creating new dataset with name {:?}", dataset.name);
    let dataset = dataset.into_inner();
    let dataset_path = match &dataset.backend {
        Backend::LocalBackend { path } => format!("{}{}", path, dataset.path),
        _ => "Not implemented".to_owned(),
    };
    debug!("Path is {}", dataset_path);
    let config_path = format!("{}/dataset.json", dataset_path);
    debug!("Config file path is {}", config_path);

    // First create the folder. (does not do anything if the folder already exists)
    fs::create_dir_all(&dataset_path).expect("Could not create directory..");

    // Store the dataset json file in the directory.
    let response_dataset = if !Path::new(&config_path).exists() {
        debug!("Creating new dataset.");
        let mut file = File::create(&config_path).unwrap();
        let string = serde_json::to_string_pretty(&dataset).unwrap();
        file.write_all(&string.as_bytes()).unwrap();
        dataset
    } else {
        debug!("Dataset already exists! Returning existing dataset.");
        let string = fs::read_to_string(&config_path).unwrap();
        serde_json::from_str(&string).unwrap()
    };

    HttpResponse::Ok().json(response_dataset)
}

#[delete("/datasets/{path}/")]
async fn delete_dataset(
    _config: web::Data<config::Config>,
    path: web::Path<String>,
) -> impl Responder {
    info!("Deleting dataset with path {:?}", path);
    HttpResponse::Ok()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file);
    cfg.service(create_dataset);
    cfg.service(delete_dataset);
}
