use std::path::PathBuf;

use thiserror::Error;

use super::parse_error::ParseError;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
    #[error("`{0}`:{1}")]
    Parse(PathBuf, ParseError),
}
