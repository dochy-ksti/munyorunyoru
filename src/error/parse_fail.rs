use pest::error::{Error as PestError, InputLocation};
use pest::RuleType;

use crate::lang::munyo_parser::Pair;

pub(crate) struct ParseFail {
    pub start_index: usize,
    pub message: String,
}

impl ParseFail {
    pub fn new(start_index: usize, message: String) -> Self {
        Self {
            start_index,
            message,
        }
    }

    pub(crate) fn from_pest_err<R: RuleType>(e: PestError<R>) -> Self {
        let start_index = match e.location {
            InputLocation::Pos(start) => start,
            InputLocation::Span((start, _end)) => start,
        };

        Self {
            start_index,
            message: format!("{e}"),
        }
    }
}

pub(crate) fn parse_fail(pair: &Pair, s: &str) -> ParseFail {
    ParseFail::new(pair.as_span().start(), s.to_string())
}

pub(crate) trait ParseFailHelper<T> {
    fn op(self, pair: &Pair) -> Result<T, ParseFail>;
    fn os(self, index: usize) -> Result<T, ParseFail>;
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
            Err(s) => Err(ParseFail::new(index, s)),
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
