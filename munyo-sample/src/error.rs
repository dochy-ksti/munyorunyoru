/// You can get this from any types implements std::fmt::Display
///
/// This can't implement std::error::Error or std::fmt::Display due to the restriction of From trait,
/// so this can't be the main Error data type of the crate.
#[derive(Debug)]
pub(crate) struct DeserializeFail {
    msg: anyhow::Error,
}

impl<D: std::fmt::Display> From<D> for DeserializeFail {
    fn from(value: D) -> Self {
        DeserializeFail {
            msg: anyhow::Error::msg(value.to_string()),
        }
    }
}

impl DeserializeFail {
    #[cfg(test)]
    pub(crate) fn msg(&self) -> String {
        // you can get the backtrace
        format!("{:?}", self.msg)
    }

    #[cfg(not(test))]
    pub(crate) fn msg(&self) -> String {
        self.msg.to_string()
    }
}
