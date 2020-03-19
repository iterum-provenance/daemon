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
}

impl Error for DaemonError {}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DaemonError::Io(err) => write!(f, "DaemonError IO: {}", err),
            DaemonError::Serialization(err) => write!(f, "DaemonError Serialization: {}", err),
            DaemonError::Cache(err) => write!(f, "DaemonError Cache: {}", err),
            DaemonError::NotFound => write!(f, "DaemonError resource could not be found."),
            DaemonError::MultipartError(err) => write!(f, "DaemonError Multipart error: {}", err),
            DaemonError::ParseError(err) => write!(f, "DaemonError ParseError: {}", err),
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
