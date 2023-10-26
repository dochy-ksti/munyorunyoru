use std::sync::OnceLock;

use pest:: Parser;
use pest_derive::Parser;

use crate::error::{MunyoResult, parse_error::ParseError};

use super::state_machine::StateMachine;

#[derive(Parser)]
#[grammar = "munyo_grammar.pest"]
pub struct MunyoParser;

use crate::error::parse_error::ParseErrorHelper;

type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub(crate) fn process_file_text(text: String) -> Result<(), ParseError> {
    let mut pairs = MunyoParser::parse(Rule::file, &text)
	.map_err(|e| ParseError(0,0, format!("{e}")))?;
    //let text = format!("{:#?}", pairs);
    //std::fs::write("sample_output.txt", text)?;

    let pair = pairs.next().unwrap();

    return parse_file(pair.into_inner());
}

fn report(pair : Pair, message : String) -> ParseError{
	let line_col = pair.line_col();
	ParseError(line_col.0, line_col.1, message)
}

fn parse_file(mut pairs: Pairs) -> Result<(), ParseError> {
	let mut state = StateMachine::new();
	let Some(tabs) = pairs.next() else{
		return Ok(())
	};
	state.indent(tabs.as_str().len()).oe(&tabs)?;
	if let Some(choice) = pairs.next(){
		match choice.as_rule(){
			Rule::main_line =>{
				parse_main_line(choice.into_inner());
			},
			Rule::empty_line =>{

			},
			Rule::commented_line =>{

			},
			_ =>{ unreachable!() }
		}
	}

    Ok(())
}

fn parse_main_line(mut pairs : Pairs) -> Result<(), ParseError>{
	let content = parse_content(pairs.next().unwrap().into_inner())?;

	Ok(())
}

fn parse_line_start_symbol(pair : Pair) -> Result<

fn parse_content(mut pairs : Pairs) -> Result<String, ParseError>{
	let mut s = String::new();
	for pair in pairs{
		match pair.as_rule(){
			Rule::char_seq =>{
				s.push_str(pair.as_str());
			},
			Rule::escaped =>{
				match pair.as_str(){
					r"\\" =>{ s.push('\\'); }
					r"\|" =>{ s.push('|'); }
					r"\n" =>{ s.push('\n'); }
					r"\r" =>{ s.push('\r'); }
					r"\t" =>{ s.push('\t'); }
					_ =>{ unreachable!() }
				}
			},
			_ =>{ unreachable!(); }
		}
	}
	Ok(s)
}