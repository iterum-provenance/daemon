use actix_web::{web, Responder, Error, HttpResponse};

use actix_multipart::Multipart;
use futures::StreamExt;
use std::time::{Instant};
use async_std::prelude::*;
use std::fs;

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

pub async fn retrieve_file(config: web::Data<config::Config>, info: web::Path<(String, String, String)>) -> HttpResponse {
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