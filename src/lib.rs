//#![allow(dead_code)]
//#![allow(unused_imports)]
#![allow(clippy::module_inception)]

mod builder;
pub mod error;
mod file_io;
mod lang;
mod test_parser;

pub use crate::file_io::read_files::read_files;
pub use crate::lang::process_file_text::process_file_text;
pub use crate::builder::default_builder::DefaultMetaBuilder;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use pest::Parser;

    use crate::{
        builder::default_builder::DefaultMetaBuilder,
        error::{MunyoResult, ReadFileError},
        lang::{
            munyo_parser::{MunyoParser, Rule},
            process_file_text::process_file_text,
        },
    };

    #[test]
    fn it_works() -> Result<(), ReadFileError> {
        let path = "sample.munyo";
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        process_file_text(unparsed_file, &DefaultMetaBuilder::new())
            .map_err(|e| ReadFileError::Parse(PathBuf::from_str(path).unwrap(), e))?;
        Ok(())
    }

    
    fn output_sample() -> Result<(), ()> {
        let path = "sample.munyo";
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        let parsed = MunyoParser::parse(Rule::file, &unparsed_file).unwrap();
        let txt = format!("{:#?}", parsed);
        fs::write("sample_output.txt", &txt).unwrap();
        Ok(())
    }

    
    fn proble_test() -> Result<(), ()> {
        // let path = "sample.munyo";
        // let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        // let parsed = ProbleParser::parse(crate::test_parser::Rule::file, &unparsed_file).unwrap();
        // let txt = format!("{:#?}", parsed);
        // fs::write("sample_output.txt", &txt).unwrap();
        Ok(())
    }
}
