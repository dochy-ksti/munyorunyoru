use std::fmt::{Debug, Display};

use super::parse_fail::ParseFail;

#[derive(Debug)]
pub(crate) enum DeserializeError {
    Fail(ParseFail),
	Msg(anyhow::Error)
}

impl DeserializeError {
    pub(crate) fn msg(s : &str) -> Self{
		Self::Msg(anyhow::anyhow!("{s}"))
	}
}

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Msg(anyhow::anyhow!("{}",msg))
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self{
			DeserializeError::Fail(fail) => write!(f, "{}", fail),
			DeserializeError::Msg(e) => write!(f, "{}", e),
		}
    }
}

impl std::error::Error for DeserializeError {}
