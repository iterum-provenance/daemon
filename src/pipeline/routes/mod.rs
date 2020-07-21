pub mod execution;
pub mod results;
use actix_web::{get, post, web, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(results::create_pipeline_result);
    cfg.service(results::get_pipeline_result);
    cfg.service(results::add_result);
    cfg.service(execution::get_pipeline_executions);
    cfg.service(execution::get_pipeline_execution);
    cfg.service(execution::create_pipeline_execution);
    cfg.service(execution::delete_pipeline_execution);
}
