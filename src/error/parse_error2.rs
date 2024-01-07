use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
/// row col message
pub struct ParseError2 {
    pub line: usize,
    pub line_str: String,
    pub message: anyhow::Error,
}

impl ParseError2 {
    pub(crate) fn new(line: usize, line_str: String, message: anyhow::Error) -> Self {
        Self {
            line,
            line_str,
            message,
        }
    }
}

impl Debug for ParseError2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n {}\n{}",
            self.line,
            &self.message,
            &self.line_str,
            &self.message.backtrace()
        )
    }
}

impl Display for ParseError2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n {}\n{}",
            self.line,
            &self.message,
            &self.line_str,
            &self.message.backtrace()
        )
    }
}

impl Error for ParseError2 {}
