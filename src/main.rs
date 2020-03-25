#[macro_use]
extern crate log;
use actix_web::{get, App, HttpServer};
use actix_web::{web, HttpResponse};
use dotenv::dotenv;
use listenfd::ListenFd;
use sled;
use std::env;

mod backend;
pub mod config;
mod dataset;
mod error;
mod utils;
mod version_control;

#[cfg(test)]
mod tests;

#[get("/")]
pub fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/1/data" method="post" enctype="multipart/form-data" target="_self">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></input>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

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

    use crate::version_control::dataset::VCDataset;
    use std::collections::HashMap;
    use std::sync::RwLock;
    let state: HashMap<String, RwLock<VCDataset>> = HashMap::new();
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
            .service(index)
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
