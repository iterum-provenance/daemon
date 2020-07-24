//! The different routes for the data versioning server, used by the CLI to communicate with the daemon.
mod branch;
mod commit;
mod dataset;
mod misc;
use actix_web::web;

/// Initializes the different routes, such that Actix exposes the endpoints
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(dataset::delete_dataset);
    cfg.service(dataset::create_dataset);
    cfg.service(dataset::get_dataset);
    cfg.service(dataset::get_datasets);
    cfg.service(branch::get_branch);
    cfg.service(branch::create_branch);
    cfg.service(commit::get_commit);
    cfg.service(commit::create_commit_with_data);
    cfg.service(misc::get_file);
    cfg.service(misc::get_vtree);
    cfg.service(misc::reset_state);
}
