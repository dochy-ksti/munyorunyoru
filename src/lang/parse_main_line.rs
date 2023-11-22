use crate::error::parse_fail::ParseFail;

use super::{
    munyo_parser::{Pair, Pairs, Rule},
    parse_content::parse_content,
    parse_line_continuation::{parse_line_continuation, set_results},
};

pub(crate) struct LineResult {
    pub(crate) content: String,
    pub(crate) params: Vec<String>,
    pub(crate) define_canceled : bool,
}

impl LineResult {
    pub(crate) fn new(content: String, params: Vec<String>, define_canceled : bool) -> Self {
        Self { content, params, define_canceled }
    }
}

pub(crate) fn parse_main_line(pairs: Pairs) -> Result<LineResult, ParseFail> {
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

    Ok(LineResult::new(content, params, line_type == LineType::Canceled))
}

fn parse_line_start_symbol(pair: Pair) -> Result<LineType, ParseFail> {
    match pair.as_str() {
        r">\" => Ok(LineType::Canceled),
        r"\>>>" => Ok(LineType::CharTriple),
        r"\>>" => Ok(LineType::CharDouble),
        r"\>" => Ok(LineType::CharSingle),
        _ => unreachable!(),
    }
}

#[derive(PartialEq)]
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

pub(crate) fn parse_param_item(pair: Pair) -> Result<String, ParseFail> {
    parse_content(pair.into_inner(), "")
}
