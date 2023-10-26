use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadFileError {
    
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
	#[error("failed to parse `{0}`, {1}")]
	Parse(PathBuf, String),
    
}