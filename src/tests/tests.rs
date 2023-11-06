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

//#[test]
fn output_sample() -> Result<(), ParseError> {
    let path = "sample.munyo";
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let r = process_file_text(unparsed_file, &DefaultMetaBuilder::new())?;
    let txt = format!("{}", r);
    fs::write("sample_output.txt", &txt).unwrap();
    Ok(())
}

#[test]
fn errors() -> Result<(), ParseError> {
    let ss = vec![
        r#"
	indented first line
		some thing
"#,
        r#"some thing
		doubly indented
"#,
        r#"name |param || one space
"#,
    ];
    let s = ss[2];
    {
        match process_file_text(s.to_string(), &DefaultMetaBuilder::new()) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
