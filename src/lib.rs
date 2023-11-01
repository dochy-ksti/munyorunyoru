//#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(clippy::module_inception)]
#![warn(unreachable_pub)]
#![deny(unused_crate_dependencies)]

mod builder;
pub mod error;
mod file_io;
mod lang;
mod test_parser;
#[cfg(test)]
mod tests;

pub use crate::builder::default_builder::DefaultMetaBuilder;
pub use crate::file_io::read_files::read_files;
pub use crate::lang::process_file_text::process_file_text;

