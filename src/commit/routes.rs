use crate::commit::Commit;
use crate::config;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::{Error, Result};
use async_std::prelude::*;
use futures::StreamExt;
use std::time::Instant;

#[post("/{dataset}/commit")]
async fn create_commit(
    _config: web::Data<config::Config>,
    dataset: web::Path<String>,
    _commit: web::Json<Commit>,
) -> impl Responder {
    info!("Posting commit to dataset {}", &dataset);

    let commit = Commit {
        hash: "".to_owned(),
        parent: String::from(""),
        branch: String::from("master"),
        name: String::from("eerste_commit"),
        desc: String::from("Dit is een commit van een dataset"),
        diff: vec![],
        deprecated: false,
    };
    // let users = User::find_all()?;
    HttpResponse::Ok().json(&commit)
}

#[post("/{dataset}/data")]
async fn create_commit_with_data(
    config: web::Data<config::Config>,
    dataset: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    info!("Posting commit with data to dataset {}", &dataset);

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

// pub async fn create(config: web::Data<config::Config>, commit: web::Json<Commit>) -> Result<String> {

//     log::info!("{:?}", &commit);

//     Ok(format!("test"))
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_commit);
    cfg.service(create_commit_with_data);
}
