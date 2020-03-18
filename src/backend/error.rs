use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BackendError {
    Io(std::io::Error),
    Serde(serde_json::error::Error),
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BackendError {
//     pub message: String,
// }

// impl BackendError {
//     pub fn new(message: String) -> BackendError {
//         BackendError { message }
//     }
// }

impl Error for BackendError {}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BackendError::Io(ref err) => write!(f, "BackendError IO: {}", err),
            BackendError::Serde(ref err) => write!(f, "BackendError Serde: {}", err),
        }
    }
}

impl From<std::io::Error> for BackendError {
    fn from(error: std::io::Error) -> BackendError {
        BackendError::Io(error)
    }
}

impl From<serde_json::error::Error> for BackendError {
    fn from(error: serde_json::error::Error) -> BackendError {
        BackendError::Serde(error)
    }
}
