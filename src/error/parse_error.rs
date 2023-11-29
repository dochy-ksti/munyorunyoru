use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
/// row col message
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub line_str: String,
    pub message: anyhow::Error,
}

impl ParseError {
    pub(crate) fn new(line: usize, col: usize, line_str: String, message: anyhow::Error) -> Self {
        Self {
            line,
            col,
            line_str,
            message,
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}\n {}\n{}",
            self.line,
            self.col,
            &self.message,
            &self.line_str,
            &self.message.backtrace()
        )
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}\n {}\n{}",
            self.line,
            self.col,
            &self.message,
            &self.line_str,
            &self.message.backtrace()
        )
    }
}

impl Error for ParseError {}
