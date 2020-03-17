use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetError {
    pub message: String,
}

impl DatasetError {
    pub fn new(message: String) -> DatasetError {
        DatasetError { message }
    }
}

impl Error for DatasetError {}

impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}
