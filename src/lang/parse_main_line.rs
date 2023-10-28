use crate::error::parse_error::ParseError;

use super::{
    munyo_parser::{Pair, Pairs, Rule},
    parse_content::{self, parse_content},
    parse_line_continuation::{parse_line_continuation, set_results},
};

pub(crate) struct LineResult {
    pub content: String,
    pub params: Vec<String>,
}

impl LineResult {
    pub(crate) fn new(content: String, params: Vec<String>) -> Self {
        Self { content, params }
    }
}

pub(crate) fn parse_main_line(pairs: Pairs) -> Result<LineResult, ParseError> {
    let mut line_type = LineType::Normal;
    let mut content = String::new();
    let mut params = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::main_line_start_symbol => {
                line_type = parse_line_start_symbol(pair)?;
            }
            Rule::content => {
                content = parse_content(pair.into_inner(), line_type.starting_text())?;
            }
            Rule::param_item => {
                params.push(parse_param_item(pair.into_inner().next().unwrap())?);
            }
            Rule::line_continuation => {
                let mut r = parse_line_continuation(pair.into_inner().next().unwrap())?;
                set_results(&mut content, &mut params, &r.content, &mut r.params)
            }
            _ => unreachable!(),
        }
    }

    match line_type {
        LineType::Canceled | LineType::Normal => {}
        LineType::CharSingle => content.push('>'),
        LineType::CharDouble => content.push_str(">>"),
        LineType::CharTriple => content.push_str(">>>"),
    }

    Ok(LineResult::new(content, params))
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

pub(crate) fn parse_param_item(pair: Pair) -> Result<String, ParseError> {
    parse_content(pair.into_inner(), "")
}
