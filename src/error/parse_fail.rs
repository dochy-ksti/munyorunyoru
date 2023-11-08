use std::fmt::Display;

use anyhow::{anyhow, Error};
use pest::error::{Error as PestError, InputLocation};
use pest::RuleType;

use crate::lang::munyo_parser::Pair;

pub struct ParseFail {
    pub(crate) start_index: usize,
    pub(crate) message: anyhow::Error,
}

impl ParseFail {
    pub(crate) fn new(start_index: usize, message: anyhow::Error) -> Self {
        Self {
            start_index,
            message,
        }
    }

    pub(crate) fn msg(start_index: usize, message: String) -> Self {
        Self {
            start_index,
            message: anyhow!(message),
        }
    }

    pub(crate) fn from_pest_err<R: RuleType>(e: PestError<R>) -> Self {
        let start_index = match e.location {
            InputLocation::Pos(start) => start,
            InputLocation::Span((start, _end)) => start,
        };

        Self {
            start_index,
            message: anyhow!("{e}"),
        }
    }
}

pub(crate) fn parse_fail(pair: &Pair, s: &str) -> ParseFail {
    ParseFail::msg(pair.as_span().start(), s.to_string())
}

pub(crate) trait ParseFailHelper<T> {
    fn op(self, pair: &Pair) -> Result<T, ParseFail>;
    fn os(self, index: usize) -> Result<T, ParseFail>;
}

pub(crate) trait ParseFailHelper2<T> {
    fn oe(self, index: usize) -> Result<T, ParseFail>;
}

impl<T> ParseFailHelper2<T> for Result<T, Error> {
    fn oe(self, index: usize) -> Result<T, ParseFail> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(ParseFail::new(index, e)),
        }
    }
}

impl<T> ParseFailHelper<T> for Result<T, String> {
    fn op(self, pair: &Pair) -> Result<T, ParseFail> {
        match self {
            Ok(r) => Ok(r),
            Err(s) => Err(parse_fail(pair, &s)),
        }
    }

    fn os(self, index: usize) -> Result<T, ParseFail> {
        match self {
            Ok(r) => Ok(r),
            Err(s) => Err(ParseFail::msg(index, s)),
        }
    }
}

pub(crate) trait PairHelper {
    fn start_index(&self) -> usize;
}

impl<'a> PairHelper for Pair<'a> {
    fn start_index(&self) -> usize {
        self.as_span().start()
    }
}

impl serde::de::Error for ParseFail {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        ParseFail::msg(0, format!("{msg}"))
    }
}

impl std::error::Error for ParseFail {}

impl std::fmt::Display for ParseFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl std::fmt::Debug for ParseFail{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n{}", &self.message, self.message.backtrace())
    }
}