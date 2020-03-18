#[macro_use]
extern crate log;
use actix_web::{error, web, HttpResponse};
use actix_web::{get, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use sled;
use std::env;

mod backend;
mod commit;
pub mod config;
mod dataset;
mod utils;

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

impl From<&dataset::Dataset> for sled::IVec {
    fn from(dataset: &dataset::Dataset) -> sled::IVec {
        bincode::serialize(dataset).unwrap().into()
    }
}

impl From<sled::IVec> for dataset::Dataset {
    fn from(ivec: sled::IVec) -> dataset::Dataset {
        bincode::deserialize(&ivec).unwrap()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let cache_path = env::var("CACHE_PATH").expect("Cache path not set");
    let t = sled::open(cache_path).unwrap();
    let dataset1 = dataset::Dataset {
        name: String::from("Dog photos"),
        path: String::from("test_dataset"),
        backend: backend::Backend::Local(backend::local::Local {
            path: String::from("./storage/"),
        }),
        description: String::from("Very important dog photos."),
    };
    // t.insert(
    //     "test_dataset".as_bytes(),
    //     bincode::serialize(&dataset1).unwrap(),
    // )
    // .unwrap();
    t.insert("test_dataset", &dataset1).unwrap();
    let result: dataset::Dataset = t.get("test_dataset").unwrap().unwrap().into();
    debug!("result: {:?}", result);

    let config = config::Config {
        app_name: String::from("Actix-web"),
        storage_path: String::from("./storage/"),
        dataset_path: String::from("./datasets/"),
        cache: t,
    };
    std::fs::create_dir_all(&config.storage_path).unwrap();

    let config_clone = config.clone();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(config_clone.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("Error when handling JSON: {:?}", err);
                error!("{}", message);
                error::InternalError::from_response(err, HttpResponse::Conflict().body(message))
                    .into()
            }))
            .service(index)
            .configure(commit::init_routes)
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
