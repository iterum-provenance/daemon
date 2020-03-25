// use std::option::NoneError;

// pub struct Error {

// }
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
