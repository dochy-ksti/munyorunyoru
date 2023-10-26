use std::sync::OnceLock;

use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

use crate::error::MunyoResult;

#[derive(Parser)]
#[grammar = "munyo_grammar.pest"]
pub struct MunyoParser;

pub(crate) fn process_file_text(text: String) -> MunyoResult<()> {
    let pairs = MunyoParser::parse(Rule::file, &text)?;
    let text = format!("{:#?}", pairs);
    std::fs::write("sample_output.txt", text)?;

    Ok(())
}
