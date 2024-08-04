#![allow(dead_code)]
/// You can get this from any types implements std::fmt::Display
///
/// This can't implement std::error::Error or std::fmt::Display due to the restriction of the From trait,
/// so this can't be the main Error type of the crate.
#[derive(Debug)]
pub(crate) struct DeserializeFail {
    pub(crate) error: anyhow::Error,
}

impl<D: std::fmt::Display> From<D> for DeserializeFail {
    fn from(value: D) -> Self {
        DeserializeFail {
            error: anyhow::Error::msg(value.to_string()),
        }
    }
}

impl DeserializeFail {
    pub(crate) fn debug(&self) -> String {
        // you can get the backtrace
        format!("{:?}", self.error)
    }

    pub(crate) fn display(&self) -> String {
        format!("{}", self.error)
    }
}
