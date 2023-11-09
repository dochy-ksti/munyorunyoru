#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::module_inception)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

mod builder;
pub mod error;
mod file_io;
mod lang;
mod serde;
mod test_parser;
#[cfg(test)]
mod tests;

pub use crate::builder::default_builder::DefaultMetaBuilder;
pub use crate::file_io::read_files::read_files;
pub use crate::lang::process_file_text::process_file_text;
pub use crate::serde::deserializer::MunyoDeserializer;
pub use crate::serde::serializer::MunyoSerializer;
pub use crate::serde::rest_arg::RestOf;

fn hoge() {
    let s = "{}";
    let j: () = serde_json::from_str(s).unwrap();
    return;
}
