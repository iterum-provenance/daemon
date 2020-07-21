#[macro_use]
extern crate log;
use actix_web::{web, HttpResponse};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod backend;
// mod cache;
pub mod config;
mod dataset;
// mod dataset_manager;
mod error;
mod pipeline;

#[cfg(test)]
mod tests;
use crate::dataset::DatasetConfig;
use iterum_rust::vc::Dataset;
use std::collections::HashMap;
use std::sync::RwLock;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let local_config_path = env::var("LOCAL_CONFIG_PATH").expect("LOCAL_CONFIG_PATH not set");
    let t = sled::open(&local_config_path).expect("Creation of local config db failed..");

    // let dataset_configs: HashMap<String, DatasetConfig> = HashMap::new();
    let mut datasets: HashMap<String, Dataset> = HashMap::new();
    let len = &t.into_iter().count();
    info!("There are {} elements in the local cache.", len);
    t.into_iter().for_each(|x| {
        let (key, value) = x.unwrap();
        info!("Loading element into cache. {:?}", std::str::from_utf8(&key).unwrap());
        let dataset_config: DatasetConfig = value.into();

        let dataset = dataset_config.read_dataset().unwrap();
        datasets.insert(dataset_config.name, dataset);
    });

    let config = web::Data::new(config::Config {
        local_config: t,
        // dataset_configs: RwLock::new(dataset_configs),
        datasets: RwLock::new(datasets),
    });

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("Error when handling JSON: {:?}", err);
                error!("{}", message);
                actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().body(message)).into()
            }))
            .configure(dataset::init_routes)
            .configure(pipeline::init_routes)
    });

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
