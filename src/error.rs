use actix_multipart::MultipartError;
use actix_web::error::ParseError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use sled;
use std::error::Error;
use std::fmt;
// use std::option::NoneError;

#[derive(Debug)]
pub enum DaemonError {
    Io(std::io::Error),
    Serialization(serde_json::error::Error),
    Cache(sled::Error),
    MultipartError(MultipartError),
    ParseError(ParseError),
    NotFound,
    AlreadyExists,
    VersionControlError(String),
}

impl Error for DaemonError {}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DaemonError::Io(err) => write!(f, "IO error: {}", err),
            DaemonError::Serialization(err) => write!(f, "Serialization error: {}", err),
            DaemonError::Cache(err) => write!(f, "Cache error: {}", err),
            DaemonError::MultipartError(err) => write!(f, "Multipart error: {}", err),
            DaemonError::ParseError(err) => write!(f, "ParseError: {}", err),
            DaemonError::NotFound => write!(f, "Resource could not be found."),
            DaemonError::AlreadyExists => write!(f, "Resource already exists."),
            DaemonError::VersionControlError(err) => write!(f, "Version control error: {}", err),
        }
    }
}

impl From<std::io::Error> for DaemonError {
    fn from(error: std::io::Error) -> DaemonError {
        match error.kind() {
            std::io::ErrorKind::NotFound => DaemonError::NotFound,
            _ => DaemonError::Io(error),
        }
    }
}

impl From<serde_json::error::Error> for DaemonError {
    fn from(error: serde_json::error::Error) -> DaemonError {
        DaemonError::Serialization(error)
    }
}

impl From<sled::Error> for DaemonError {
    fn from(error: sled::Error) -> DaemonError {
        DaemonError::Cache(error)
    }
}

impl From<MultipartError> for DaemonError {
    fn from(error: MultipartError) -> DaemonError {
        DaemonError::MultipartError(error)
    }
}

impl From<ParseError> for DaemonError {
    fn from(error: ParseError) -> DaemonError {
        DaemonError::ParseError(error)
    }
}

impl ResponseError for DaemonError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            DaemonError::NotFound => StatusCode::NOT_FOUND,
            DaemonError::VersionControlError(_) | DaemonError::AlreadyExists => {
                StatusCode::CONFLICT
            }
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
