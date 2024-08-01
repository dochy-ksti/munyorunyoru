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
		// backtrace が得られる
		format!("{:?}", self.msg)
	}

	#[cfg(not(test))]
	pub(crate) fn msg(&self) -> String {
		self.msg.to_string()
	}
}