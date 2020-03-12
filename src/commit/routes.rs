use crate::backend::storable::Storable;
use crate::commit::ChangeType;
use crate::commit::Commit;
use crate::config;
use crate::dataset::Dataset;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::{Error, Result};
use async_std::prelude::*;
use bytes::Bytes;
use futures::StreamExt;
use std::fs;
use std::time::Instant;

#[post("/{dataset}/blablabla")]
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
    HttpResponse::Ok().json(&commit)
}

#[post("/{dataset}/commit")]
async fn create_commit_with_data(
    config: web::Data<config::Config>,
    dataset_string: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    info!("Posting commit with data to dataset {}", &dataset_string);

    // First find the dataset to which the commit is posted.
    let dataset_path = format!("{}{}", &config.storage_path, dataset_string);
    let dataset = Dataset::read_from_path(&dataset_path).unwrap();
    debug!("{:?}", dataset);

    // iterate over multipart stream
    let now = Instant::now();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_type();
        debug!("Received content type: {}", content_type);

        let temp_path = format!("./tmp/");
        fs::create_dir_all(&temp_path).expect("Could not create temporary file directory.");

        match (content_type.type_(), content_type.subtype()) {
            (mime::APPLICATION, mime::JSON) => {
                // Parse the commit
                let mut commit_bytes: Vec<u8> = vec![];
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    commit_bytes.extend(data.to_vec());
                }
                let commit_string = String::from_utf8(commit_bytes).unwrap();
                let commit: Commit = serde_json::from_str(&commit_string).unwrap();
                let filepath = format!("{}commit.json", &temp_path);
                let mut f = async_std::fs::File::create(filepath).await?;
                f.write(serde_json::to_string_pretty(&commit).unwrap().as_bytes())
                    .await?;
            }
            (mime::IMAGE, mime::JPEG) => {
                let content_disp = field
                    .content_disposition()
                    .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
                debug!("Received content disp: {}", content_disp);
                let filename = content_disp
                    .get_filename()
                    .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

                let filepath = format!("{}{}", &temp_path, &filename);
                debug!("Saving file to {}", filepath);
                let mut f = async_std::fs::File::create(filepath).await?;
                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f.write_all(&data).await?;
                }
            }
            _ => debug!("Not implemented"),
        }
    }
    debug!("Time to upload file \t{}ms", now.elapsed().as_millis());

    // Now store the data in the actual backend.
    dataset
        .backend
        .store_commit(&dataset, "./tmp/".to_string())
        .unwrap();
    Ok(HttpResponse::Ok().into())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_commit);
    cfg.service(create_commit_with_data);
}
