pub mod execution;
mod helpers;
pub mod provenance;
pub mod results;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(results::create_pipeline_result);
    cfg.service(results::get_pipeline_result);
    cfg.service(results::add_result);
    cfg.service(execution::get_pipeline_executions);
    cfg.service(execution::get_dataset_pipeline_executions);
    cfg.service(execution::get_pipeline_execution);
    cfg.service(execution::get_pipeline_execution_without_dataset);
    cfg.service(execution::create_pipeline_execution);
    cfg.service(execution::delete_pipeline_execution);
    cfg.service(provenance::post_fragment_lineage);
    cfg.service(provenance::get_fragment_lineages);
    cfg.service(provenance::get_fragment_lineage);
}
