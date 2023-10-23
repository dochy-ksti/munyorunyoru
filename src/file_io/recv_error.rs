use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadFilesError {
    
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
    
}