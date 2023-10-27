mod converters;
pub mod error;
mod file_io;
mod lang;

pub use crate::file_io::read_files::read_files;
pub use crate::lang::process_file_text::process_file_text;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use pest::Parser;

    use crate::{
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
        process_file_text(unparsed_file)
            .map_err(|e| ReadFileError::Parse(PathBuf::from_str(path).unwrap(), e))?;
        Ok(())
    }

    #[test]
    fn output_sample() -> Result<(), ()> {
        let path = "sample.munyo";
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        let parsed = MunyoParser::parse(Rule::file, &unparsed_file).unwrap();
        let txt = format!("{:#?}", parsed);
        fs::write("sample_output.txt", &txt).unwrap();
        Ok(())
    }
}
