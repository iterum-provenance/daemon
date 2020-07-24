//! The daemon repository, which combines the *storage interface*, and the *data versioning server* of **Iterum**.
//! In general, the *data versioning server* resides in the `dataset` submodule, and the *storage interface* resides in the rest of the submodules.

#[macro_use]
extern crate log;
use actix_web::{web, HttpResponse};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
mod backend;
mod config;
mod dataset;
mod error;
mod pipeline;

use crate::dataset::DatasetConfig;
use iterum_rust::vc::Dataset;
use std::collections::HashMap;
use std::sync::RwLock;

/// Main initializes the daemon by setting up an actix server to expose various endpoints to be used by the other components in **Iterum**.
///
/// It uses a local key-value store stored in LOCAL_CONFIG_PATH to store the various configured datasets (The names of the dataset + where this dataset is stored).
/// It reads the various datasets, and loads them into a HashMap in memory, which is shared between the actix workers.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // Open local kv-store if it exists, otherwise create one.
    let local_config_path = env::var("LOCAL_CONFIG_PATH").expect("LOCAL_CONFIG_PATH not set");
    let t = sled::open(&local_config_path).expect("Creation of local config db failed..");

    // Load each dataset into memory
    let mut datasets: HashMap<String, Dataset> = HashMap::new();
    let len = &t.into_iter().count();
    info!("Loading {} elements in the local cache.", len);
    t.into_iter().for_each(|x| {
        let (key, value) = x.unwrap();
        info!("Loading element into cache. {:?}", std::str::from_utf8(&key).unwrap());
        let dataset_config: DatasetConfig = value.into();

        let dataset = dataset_config.read_dataset().unwrap();
        datasets.insert(dataset_config.name, dataset);
    });

    // Initialize shared config between actix workers
    let config = web::Data::new(config::Config {
        local_config: t,
        datasets: RwLock::new(datasets),
    });

    // Configure actix server
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("Error when handling JSON: {:?}", err);
                error!("{}", message);
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().body(message)).into()
            }))
            .configure(pipeline::init_routes)
            .configure(dataset::init_routes)
    });

    // Start actix server
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST not set");
            let port = env::var("PORT").expect("PORT not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting Iterum Daemon");
    server.run().await
}
