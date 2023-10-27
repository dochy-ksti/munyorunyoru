use crate::error::parse_error::ParseError;

use super::{munyo_parser::{Pair, Pairs, Rule}, parse_content::{self, parse_content}};

fn parse_main_line(mut pairs: Pairs) -> Result<(), ParseError> {
    let mut line_type = LineType::Normal;
	let mut content = String::new();
	let mut param_items = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::main_line_start_symbol => {
                line_type = parse_line_start_symbol(pair)?;
            }
			Rule::content =>{
				content = parse_content(pair.into_inner(), line_type.starting_text())?;
			}
			Rule::param_item =>{

			}
			Rule::line_continuation =>{

			}
			_ =>unreachable!(),
        }
    }
    

    Ok(())
}

fn parse_line_start_symbol(pair: Pair) -> Result<LineType, ParseError> {
    match pair.as_str() {
        r">\" => Ok(LineType::Canceled),
        r"\>>>" => Ok(LineType::CharTriple),
        r"\>>" => Ok(LineType::CharDouble),
        r"\>" => Ok(LineType::CharSingle),
        _ => unreachable!(),
    }
}

pub(crate) enum LineType {
    Normal,
    Canceled,
    CharSingle,
    CharDouble,
    CharTriple,
}

impl LineType {
    pub(crate) fn starting_text(&self) -> &str {
        match self {
            Self::Normal | Self::Canceled => "",
            Self::CharSingle => ">",
            Self::CharDouble => ">>",
            Self::CharTriple => ">>>",
        }
    }
}

fn parse_param_item(pair: Pair) -> Result<String, ParseError> {
    parse_content(pair.into_inner(), "")
}
