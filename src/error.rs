use std::fmt::{Debug, Display, Formatter};

use anyhow::anyhow;

pub(crate) mod read_file_error;
pub(crate) mod parse_error;

pub use read_file_error::ReadFileError;

/// The error type.
///
/// This wraps anyhow::Error. You can get it from Into trait.
///
/// Maybe the source error retrieved from anyhow::Error can be used to determine the cause of the error,
/// but there's no guarantees about the inner error format.
pub struct MunyoError {
    error: anyhow::Error,
}

pub type MunyoResult<T> = Result<T, MunyoError>;

impl MunyoError {
    pub(crate) fn new(e: impl Into<anyhow::Error>) -> Self {
        Self { error: e.into() }
    }
}

impl Display for MunyoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for MunyoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Into<anyhow::Error> for MunyoError {
    fn into(self) -> anyhow::Error {
        self.error
    }
}

impl From<anyhow::Error> for MunyoError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<std::io::Error> for MunyoError {
    fn from(e: std::io::Error) -> Self {
        Self::new(e)
    }
}

impl From<async_channel::RecvError> for MunyoError {
    fn from(e: async_channel::RecvError) -> Self {
        Self::new(e)
    }
}

impl From<async_channel::TryRecvError> for MunyoError {
    fn from(e: async_channel::TryRecvError) -> Self {
        Self::new(e)
    }
}

impl From<&str> for MunyoError {
    fn from(e: &str) -> Self {
        Self::new(anyhow!("{}", e))
    }
}

impl From<String> for MunyoError {
    fn from(e: String) -> Self {
        Self::new(anyhow!("{}", e))
    }
}

impl<T: Debug + Copy + core::hash::Hash + Ord + Send + Sync + 'static> From<pest::error::Error<T>>
    for MunyoError
{
    fn from(e: pest::error::Error<T>) -> Self {
        Self::new(e)
    }
}
