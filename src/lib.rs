#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
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

pub use crate::builder::default_builder::DefaultMetaBuilder;
pub use crate::lang::from_str_with_metabuilder::from_str_with_metabuilder;
pub use crate::pub_apis::{from_str, to_string, from_file};
pub use crate::serde::deserializer::MunyoDeserializer;
pub use crate::serde::rest_of::RestOf;
pub use crate::serde::serializer::MunyoSerializer;
pub use crate::file_io::concurrent::Concurrent;

pub use crate::error::munyo_error::Error;
pub type Result<T> = std::result::Result<T, Error>;
