use std::sync::OnceLock;

use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

#[derive(Parser)]
#[grammar = "munyo_grammar.pest"]
pub struct MunyoParser;

pub(crate) fn process_file_text(text: String) -> Result<(), String> {
    match MunyoParser::parse(Rule::file, &text) {
        Ok(a) => Ok(()),
        Err(e) => return Err(format!("{e}")),
    }
}
