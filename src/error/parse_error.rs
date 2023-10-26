use pest::{iterators::Pair, RuleType};

/// row col message
pub(crate) struct ParseError(pub usize, pub usize, pub String);

impl ParseError{
	pub fn new(row : usize, col : usize, message : String) -> ParseError{
		ParseError(row, col, message)
	}
}

pub(crate) trait ParseErrorHelper<T>{
	fn oe<R : RuleType>(self, pair : &Pair<'_, R>) -> Result<T, ParseError>;
}

impl<T> ParseErrorHelper<T> for Result<T, String>{
    fn oe<R : RuleType>(self, pair : &Pair<'_, R>) -> Result<T, ParseError> {
        match self{
			Ok(r) => Ok(r),
			Err(s) =>{ 
				let line_col = pair.line_col();
				Err(ParseError(line_col.0, line_col.1, s))
			}
		}
    }
}

