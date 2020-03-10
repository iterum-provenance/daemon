use crate::config;
use crate::dataset::Dataset;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/{dataset}/{commit}/{file}")]
async fn get_file(
    _config: web::Data<config::Config>,
    info: web::Path<(String, String, String)>,
) -> impl Responder {
    info!("Retrieving file {} from {}:{}", info.2, info.0, info.1);

    HttpResponse::Ok()
}

#[post("/datasets/")]
async fn create_dataset(
    _config: web::Data<config::Config>,
    dataset: web::Json<Dataset>,
) -> impl Responder {
    info!("Creating new dataset with name {:?}", dataset.name);
    HttpResponse::Ok()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file);
    cfg.service(create_dataset);
}
