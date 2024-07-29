#![allow(clippy::module_inception)]
#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(missing_docs)]

//! Munyo is a language.
//! Since the API documentation isn't enough to use it, I wrote various materials.
//!
//! See [readme](https://github.com/dochy-ksti/munyorunyoru/blob/master/readme.md)
//! [samples](https://github.com/dochy-ksti/munyorunyoru/tree/master/src/samples)
//! [lang_spec](https://github.com/dochy-ksti/munyorunyoru/blob/master/lang_spec.txt)
//! [What's DSL?](https://github.com/dochy-ksti/munyorunyoru/blob/master/whats_dsl.md)

pub mod builder;
pub mod error;
pub mod file_io;
pub mod lang;
mod pub_apis;

mod serde;
#[cfg(test)]
mod tests;

pub mod samples;

#[doc(inline)]
pub use crate::builder::default_builder::MunyoItem;
pub use crate::file_io::concurrent::Concurrent;
pub use crate::lang::from_str_with_metabuilder::from_str_with_metabuilder;
pub use crate::pub_apis::{from_file, from_str, from_str_with_path, to_string};
pub use crate::serde::deserializer::MunyoDeserializer;
pub use crate::serde::rest_of::{IgnoredAnyVisitor, RestOf};
pub use crate::serde::serializer::MunyoSerializer;

pub use crate::error::munyo_error::Error;
/// Result type of Munyo
pub type Result<T> = std::result::Result<T, Error>;

use std::path::Path;
pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> crate::Result<String> {
    std::fs::read_to_string(&path)
        .map_err(|e| crate::Error::ReadFile(path.as_ref().to_path_buf(), e.into()))
}

#[doc(hidden)]
/// This is only meant for testing.
///
/// The created file survives until the NamedTempFile drops
pub fn temp(s: &str) -> std::io::Result<tempfile::NamedTempFile> {
    use std::io::{Seek, SeekFrom, Write};
    let mut t = tempfile::NamedTempFile::new()?;
    writeln!(t, "{s}")?;
    t.seek(SeekFrom::Start(0))?;
    Ok(t)
}
