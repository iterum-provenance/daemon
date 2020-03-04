use actix_web::{web, Responder, Error, HttpResponse};

use actix_multipart::Multipart;
use futures::StreamExt;
use std::time::{Instant};
use async_std::prelude::*;

use super::config;


pub async fn save_file(config: web::Data<config::Config>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let now = Instant::now();

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("{}/{}", &config.storage_path, filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    println!("Time to upload file \t{}ms", now.elapsed().as_millis());

    Ok(HttpResponse::Ok().into())
}

pub async fn index(data: web::Data<config::Config>, info: web::Path<(String, String, String)>) -> impl Responder {
    let string = format!("Serving file with path {}/{}/{}", info.0, info.1, info.2);
    let app_name = &data.app_name;
    // let response = format!("Hello {}! id:{}", info.1, info.0);
    println!("{} {}", app_name, string);
    HttpResponse::Ok().body(string)
}

pub fn _index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}