use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct VCErrorMessage {
    message: String,
}

impl VCErrorMessage {
    pub fn new(message: String) -> VCErrorMessage {
        VCErrorMessage { message }
    }
}

#[derive(Debug, PartialEq)]
pub enum VersionControlError {
    CommitIncomplete(VCErrorMessage),
    ParentCommitNotFound,
    BranchNotFound,
    CommitHashAlreadyExists,
    ParentCommitIsNotBranchHead,
}

impl Error for VersionControlError {}

impl fmt::Display for VersionControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionControlError::ParentCommitNotFound => {
                write!(f, "Parent of commit is not present in the version tree.")
            }
            VersionControlError::CommitHashAlreadyExists => {
                write!(f, "Commit hash already exists.")
            }
            _ => write!(f, "Error handling not implemented yet."),
        }
    }
}

impl ResponseError for VersionControlError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            VersionControlError::CommitIncomplete(_) => StatusCode::CONFLICT,
            VersionControlError::ParentCommitNotFound => StatusCode::NOT_FOUND,
            VersionControlError::BranchNotFound => StatusCode::NOT_FOUND,
            VersionControlError::CommitHashAlreadyExists => StatusCode::CONFLICT,
            VersionControlError::ParentCommitIsNotBranchHead => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = format!("{}", self);
        // match self {
        //     DaemonError::Io(err) => format!("{}", err),
        //     DaemonError::Serialization(err) => format!("{}", err),
        //     DaemonError::Cache(err) => format!("{}", err),
        //     DaemonError::NotFound(err) => format!("{}", err),
        // };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
