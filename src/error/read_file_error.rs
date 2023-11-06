use std::{fmt::Display, path::PathBuf};

use thiserror::Error;

use super::parse_error::ParseError;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
    #[error("`{0}`:{1}")]
    Parse(PathBuf, ParseError),
    #[error("{0}")]
    Deserialize(String),
    #[error("{0}")]
    Serialize(String),
}

impl serde::de::Error for ReadFileError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Deserialize(format!("{}", msg))
    }
}

impl serde::ser::Error for ReadFileError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Serialize(format!("{msg}"))
    }
}
