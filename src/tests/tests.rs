use std::{fs, path::PathBuf, str::FromStr};

use crate::{
    builder::default_builder::DefaultMetaBuilder,
    error::{munyo_error::PathItem, parse_error::ParseError},
    lang::process_file_text::process_file_text,
    Error,
};

#[test]
fn it_works() -> Result<(), Error> {
    let path = "sample.munyo";
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let r = process_file_text(unparsed_file, &DefaultMetaBuilder)
        .map_err(|e| Error::Parse(PathItem::new(PathBuf::from_str(path).ok()), e))?;

    println!("{}", r);

    Ok(())
}

//#[test]
fn output_sample() -> Result<(), ParseError> {
    let path = "sample.munyo";
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let r = process_file_text(unparsed_file, &DefaultMetaBuilder)?;
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
        match process_file_text(s.to_string(), &DefaultMetaBuilder) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}
