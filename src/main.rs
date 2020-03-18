#[macro_use]
extern crate log;
use actix_web::{error, web, HttpResponse};
use actix_web::{get, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let config = config::Config {
        app_name: String::from("Actix-web"),
        storage_path: String::from("./storage/"),
        dataset_path: String::from("./datasets/"),
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
