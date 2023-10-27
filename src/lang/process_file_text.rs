use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::{parse_err, ParseError},
};

use super::{
    line_type::LineType,
    munyo_parser::{MunyoParser, Pair, Pairs, Rule},
    parse_line_contents::parse_line_contents,
    state::State, parse_content::parse_content,
};

use crate::error::parse_error::ParseErrorHelper;
use pest::Parser;

pub fn process_file_text<MB, B, T>(text: String, builder: &MB) -> Result<(), ParseError>
where
    MB: MetaBuilder<B, T>,
    B: Builder<T>,
{
    let mut pairs =
        MunyoParser::parse(Rule::file, &text).map_err(|e| ParseError::from_pest_err(e))?;

    let pair = pairs.next().unwrap();

    return parse_file(pair.into_inner(), builder);
}

fn parse_file<MB, B, T>(mut pairs: Pairs, builder: &MB) -> Result<(), ParseError>
where
    MB: MetaBuilder<B, T>,
    B: Builder<T>,
{
    let mut state = State::new();
	let mut indent_level = 0;
    while let Some(choice) = pairs.next() {
        match choice.as_rule() {
            Rule::tabs => {
				indent_level = choice.as_str().len();
            }
            Rule::line_contents => {
                parse_line_contents(choice.into_inner().next().unwrap(), indent_level, &mut state, builder)?;
            }
            Rule::new_line => {}
            Rule::EOI => {
                return Ok(());
            }
            _ => {
                unreachable!()
            }
        }
    }

    unreachable!()
}


pub(crate) fn parse_new_line(pair: Pair) -> String {
    pair.as_str().to_string()
}
