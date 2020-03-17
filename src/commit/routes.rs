use crate::backend::storable::Storable;
use crate::commit::ChangeType;
use crate::commit::Commit;
use crate::config;
use crate::dataset::Dataset;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::{Error, Result};
use async_std::prelude::*;
use bytes::Bytes;
use futures::StreamExt;
use std::fs;
use std::time::Instant;

#[get("/{dataset}/commit/{commit}")]
async fn get_commit(
    _config: web::Data<config::Config>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    info!("Getting commit from dataset with path {:?}", path.0);
    match Dataset::get_by_path(&path.0) {
        Ok(dataset) => match dataset.get_commit(&path.1) {
            Ok(commit) => HttpResponse::Ok().json(commit),
            Err(e) => HttpResponse::NotFound().finish(),
        },
        _ => HttpResponse::NotFound().finish(),
    }
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
    let dataset = Dataset::read_from_path(&dataset_path)
        .expect("Something went wrong reading the dataset file..");
    debug!("{:?}", dataset);

    // iterate over multipart stream
    let now = Instant::now();
    let temp_path = format!("./tmp/");
    fs::create_dir_all(&temp_path).expect("Could not create temporary file directory.");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_type();

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

    // Parse the files stored in the temporary folder. Return the commit data structure.
    let commit = dataset
        .backend
        .get_commit_from_file("./tmp/".to_string())
        .unwrap();

    dataset
        .backend
        .store_committed_files(&dataset, "./tmp/".to_string())
        .unwrap();
    // Now add the commit to the dataset.

    match dataset.add_commit(&commit) {
        Ok(()) => Ok(HttpResponse::Ok().json(&commit)),
        Err(e) => Ok(HttpResponse::Conflict().json(e)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(create_commit);
    cfg.service(create_commit_with_data);
    cfg.service(get_commit);
}
