use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::{parse_err, ParseError},
};

use super::{
    line_type::LineType,
    munyo_parser::{MunyoParser, Pair, Pairs, Rule},
    parse_line_contents::parse_line_contents,
    state_machine::StateMachine,
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
    let mut state = StateMachine::new();
	let mut indent_level = 0;
    while let Some(choice) = pairs.next() {
        match choice.as_rule() {
            Rule::tabs => {
				indent_level = choice.as_str().len();
            }
            Rule::line_contents => {
                parse_line_contents(choice.into_inner(), indent_level, &mut state, builder)?;
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

fn parse_main_line(mut pairs: Pairs) -> Result<(), ParseError> {
    let first = pairs.next().unwrap();
    let (line_type, content) = match first.as_rule() {
        Rule::main_line_start_symbol => {
            let line_type = parse_line_start_symbol(first)?;
            (line_type, pairs.next().unwrap())
        }
        Rule::content => (LineType::Normal, first),
        _ => {
            unreachable!()
        }
    };
    let content = parse_content(
        pairs.next().unwrap().into_inner(),
        line_type.starting_text(),
    )?;

    let mut params: Vec<String> = vec![];

    let p = loop {
        let p = pairs.next().unwrap();
        match p.as_rule() {
            Rule::param_item => params.push(parse_param_item(p)?),
            //Rule::line_end => break p,
            _ => {
                unreachable!()
            }
        }
    };

    Ok(())
}

fn parse_line_start_symbol(pair: Pair) -> Result<LineType, ParseError> {
    match pair.as_str() {
        ">>>" => Err(parse_err(&pair, ">>> is not supported")),
        ">>" => Ok(LineType::Double),
        r">\" => Ok(LineType::Normal),
        ">" => Ok(LineType::Single),
        r"\>>>" => Ok(LineType::CharTriple),
        r"\>>" => Ok(LineType::CharDouble),
        r"\>" => Ok(LineType::CharSingle),
        _ => unreachable!(),
    }
}

fn parse_content(mut pairs: Pairs, starting_text: &str) -> Result<String, ParseError> {
    let mut s = String::with_capacity(8);
    s.push_str(starting_text);
    for pair in pairs {
        match pair.as_rule() {
            Rule::char_seq => {
                s.push_str(pair.as_str());
            }
            Rule::escaped => match pair.as_str() {
                r"\\" => {
                    s.push('\\');
                }
                r"\|" => {
                    s.push('|');
                }
                r"\n" => {
                    s.push('\n');
                }
                r"\r" => {
                    s.push('\r');
                }
                r"\t" => {
                    s.push('\t');
                }
                _ => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
    Ok(s)
}

fn parse_param_item(pair: Pair) -> Result<String, ParseError> {
    parse_content(pair.into_inner(), "")
}

fn parse_new_line(pair: Pair) -> String {
    pair.as_str().to_string()
}
