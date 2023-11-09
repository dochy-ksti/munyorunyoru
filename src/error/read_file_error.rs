use std::{fmt::Display, path::PathBuf};

use thiserror::Error;

use super::parse_error::ParseError;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
    #[error("`{0}`:{1}")]
    Parse(PathItem, ParseError),
    #[error("`{0}`:{1}")]
    Deserialize(PathItem, ParseError),
    #[error("{0}")]
    DeserializeCustom(String),
    #[error("{0}")]
    Serialize(anyhow::Error),
    #[error("{0}")]
    SerializeCustom(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PathItem {
    pub path: Option<PathBuf>,
}

impl PathItem {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }
}

impl Display for PathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(t) => write!(f, "{}", t.to_string_lossy()),
            None => Ok(()),
        }
    }
}

impl serde::de::Error for ReadFileError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::DeserializeCustom(format!("{}", msg))
    }
}

impl serde::ser::Error for ReadFileError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerializeCustom(format!("{msg}"))
    }
}
