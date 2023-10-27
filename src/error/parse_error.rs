use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use pest::error::Error as PestError;
use pest::{error::LineColLocation, RuleType};

use crate::lang::munyo_parser::Pair;

/// row col message
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub line_str: String,
    pub message: String,
}

impl ParseError {
    pub(crate) fn new(line: usize, col: usize, line_str: String, message: String) -> Self {
        Self {
            line,
            col,
            line_str,
            message,
        }
    }

    pub(crate) fn from_pest_err<R: RuleType>(e: PestError<R>) -> Self {
        let (line, col) = match e.line_col {
            LineColLocation::Pos(linecol) => linecol,
            LineColLocation::Span(lc, _) => lc,
        };
        Self {
            line,
            col,
            line_str: e.line().to_string(),
            message: format!("{e}"),
        }
    }
}

pub(crate) fn parse_err(pair: &Pair, s: &str) -> ParseError {
    let line_col = pair.line_col();
    ParseError::new(
        line_col.0,
        line_col.1,
        pair.as_str().to_string(),
        s.to_string(),
    )
}

pub(crate) trait ParseErrorHelper<T> {
    fn oe(self, pair: &Pair) -> Result<T, ParseError>;
}

impl<T> ParseErrorHelper<T> for Result<T, String> {
    fn oe(self, pair: &Pair) -> Result<T, ParseError> {
        match self {
            Ok(r) => Ok(r),
            Err(s) => Err(parse_err(pair, &s)),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}\n{}",
            self.line, self.col, &self.message, &self.line_str
        )
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {} | {}",
            self.line, self.col, &self.message, &self.line_str
        )
    }
}

impl Error for ParseError {}
