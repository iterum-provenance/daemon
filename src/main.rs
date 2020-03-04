use std::sync::Mutex;
use actix_web::{get, web, App, HttpServer, Responder, Error, HttpRequest, HttpResponse};
use actix_web::middleware::Logger;
use std::fs;
use std::path::Path;
use listenfd::ListenFd;
use serde::Deserialize;
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;
use std::time::{Duration, Instant};
use async_std::prelude::*;

pub mod handles;
pub mod config;



async fn retrieve_file(config: web::Data<config::Config>, info: web::Path<(String, String, String)>) -> HttpResponse {
    let file_path = format!("{}/{}", &config.storage_path, &info.2);

    // Read file, check if it is available. If it is, return it. Else return a 404.
    match fs::read(&file_path) {
        Ok(contents) => {
            let response = HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(contents);
            println!("{:?}", response);
            response
        },
        Err(error) => {
            println!("{}", error);
            HttpResponse::NotFound()
                .body("Could not find the file")
        }
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let config = config::Config {
        app_name: String::from("Actix-web"),
        storage_path: String::from("./storage/"),
    };

    std::fs::create_dir_all(&config.storage_path).unwrap();
    let mut listenfd = ListenFd::from_env();
    
    let mut server = HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .route("/", web::get().to(handles::_index))
            .route("/", web::post().to(handles::save_file))
            .route("/{dataset}/{commit}/{filename}", web::get().to(retrieve_file))
        });

    
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await

}