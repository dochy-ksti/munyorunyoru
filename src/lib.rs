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

pub use crate::builder::default_builder::DefaultMetaBuilder;
pub use crate::file_io::read_files::read_files;
pub use crate::lang::process_file_text::process_file_text;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::{
        builder::default_builder::DefaultMetaBuilder,
        error::{parse_error::ParseError, ReadFileError},
        lang::process_file_text::process_file_text,
    };

    #[test]
    fn it_works() -> Result<(), ReadFileError> {
        let path = "sample.munyo";
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        let r = process_file_text(unparsed_file, &DefaultMetaBuilder::new())
            .map_err(|e| ReadFileError::Parse(PathBuf::from_str(path).unwrap(), e))?;

        println!("{}", r);

        Ok(())
    }

    #[test]
    fn output_sample() -> Result<(), ParseError> {
        let path = "sample.munyo";
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        let r = process_file_text(unparsed_file, &DefaultMetaBuilder::new())?;
        let txt = format!("{}", r);
        fs::write("sample_output.txt", &txt).unwrap();
        Ok(())
    }
}
