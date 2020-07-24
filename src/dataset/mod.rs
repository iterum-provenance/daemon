//! Contains routes and models with regards to data versioning, and retrieving and storing files for a dataset.
pub mod models;
pub mod routes;
pub use models::DatasetConfig;
pub use routes::init_routes;
