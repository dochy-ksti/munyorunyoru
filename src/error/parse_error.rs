use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
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
