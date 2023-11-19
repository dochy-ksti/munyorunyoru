use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(crate) struct SerdeInnerError {
    msg: String,
}

impl serde::ser::Error for SerdeInnerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            msg: format!("{}", msg),
        }
    }
}

impl serde::de::Error for SerdeInnerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            msg: format!("{}", msg),
        }
    }
}

impl Display for SerdeInnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl std::error::Error for SerdeInnerError {}
