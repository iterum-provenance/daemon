use crate::config;
use crate::dataset::{Backend, Dataset};
use actix_web::{error, get, post, web, HttpResponse, Responder};
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

    if !Path::new(&config_path).exists() {
        let mut file = File::create(&config_path).unwrap();
        let dataset_json = serde_json::to_string_pretty(&dataset).unwrap();
        file.write_all(&dataset_json.as_bytes()).unwrap();
    } else {
        debug!("Dataset already exists!");
    };
    // debug!("{:?}", entries);
    // for entry in entries {}

    // for entry in entries {
    //     let entry = entry?;
    //     let path = entry.path();
    //     match path.extension() {
    //         Some(dataset) => println!("This is a database file"),
    //         None => println!("This is not a database file"),
    //     }
    //     let filename = path.file_name().unwrap();
    //     println!("Filename: {}", &filename.to_str().unwrap());

    //     // println!("Name: {}", path.unwrap().path().display())
    //     // println!("Name: {}", path.unwrap().path().display())
    // }

    // If not, create a new directory for this dataset

    // Store the dataset json file in the directory.
    HttpResponse::Ok()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file);
    cfg.service(create_dataset);
}
