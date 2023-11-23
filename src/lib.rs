#![allow(clippy::module_inception)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

pub mod builder;
pub mod error;
pub mod file_io;
mod lang;
mod pub_apis;
pub mod samples;
mod serde;
mod test_parser;
#[cfg(test)]
mod tests;

pub use crate::builder::default_builder::MunyoItem;
pub use crate::file_io::concurrent::Concurrent;
pub use crate::lang::from_str_with_metabuilder::from_str_with_metabuilder;
pub use crate::pub_apis::{from_file, from_str, from_str_with_path, to_string};
pub use crate::serde::deserializer::MunyoDeserializer;
pub use crate::serde::rest_of::RestOf;
pub use crate::serde::serializer::MunyoSerializer;

pub use crate::error::munyo_error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use std::path::Path;
pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> crate::Result<String> {
    std::fs::read_to_string(&path)
        .map_err(|e| crate::Error::ReadFile(path.as_ref().to_path_buf(), e.to_string()))
}
