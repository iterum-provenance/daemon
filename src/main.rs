#[macro_use]
extern crate log;
use crate::version_control::dataset::VCDataset;
use actix_web::{get, App, HttpServer};
use actix_web::{web, HttpResponse};
use dotenv::dotenv;
use listenfd::ListenFd;
use sled;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

mod backend;
pub mod config;
mod dataset;
mod error;
mod utils;
mod version_control;

#[cfg(test)]
mod tests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let cache_path = env::var("CACHE_PATH").expect("Cache path not set");

    // Delete the cache if it exists. (For development purposes.)
    // if std::path::Path::new(&cache_path).exists() {
    //     std::fs::remove_dir_all(&cache_path).unwrap();
    //     // std::fs::remove_dir_all(String::from("./storage/")).unwrap();
    // }
    let t = sled::open(&cache_path).expect("Creation of cache db failed..");

    let state = match t.get(b"dbs").unwrap() {
        Some(ivec) => ivec.into(),
        None => {
            let dbs = config::MemoryCache {
                dbs: RwLock::new(HashMap::new()),
            };
            t.insert(b"dbs", &dbs).unwrap();
            dbs
        }
    };
    // if t.contains_key(b"dbs") {
    //     let state: config::MemoryCache = ;
    // } else {
    //     t.get(b"dbs").unwrap().unwrap()
    // }
    // match t.get(b"dbs").unwrap() ;

    let config = web::Data::new(config::Config {
        cache: t,
        state: state,
    });

    // std::fs::create_dir_all(&config.storage_path).unwrap();

    // let config_clone = config.clone();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("Error when handling JSON: {:?}", err);
                error!("{}", message);
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::Conflict().body(message),
                )
                .into()
            }))
            .configure(dataset::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting Iterum Daemon");
    server.run().await
}
