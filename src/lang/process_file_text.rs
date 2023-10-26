use std::sync::OnceLock;

use pest::{iterators::{Pairs, Pair}, Parser};
use pest_derive::Parser;

use crate::error::{MunyoResult, parse_error::ParseError};

use super::state_machine::StateMachine;

#[derive(Parser)]
#[grammar = "munyo_grammar.pest"]
pub struct MunyoParser;

pub(crate) fn process_file_text(text: String) -> Result<(), ParseError> {
    let mut pairs = MunyoParser::parse(Rule::file, &text);
    //let text = format!("{:#?}", pairs);
    //std::fs::write("sample_output.txt", text)?;

    let pair = pairs.next().unwrap();

    return parse_file(pair.into_inner());
}

fn report(pair : Pair<'_, Rule>, message : String) -> ParseError{
	let line_col = pair.line_col();
	ParseError(line_col.0, line_col.1, message)
}

fn parse_file(mut pairs: Pairs<'_, Rule>) -> Result<(), ParseError> {
	let mut state = StateMachine::new();
	let tabs = pairs.next().unwrap();
	let hoge =  state.indent(tabs.as_str().len()).e(&tabs)?;

    Ok(())
}
