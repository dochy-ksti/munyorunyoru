use std::{path::PathBuf, fmt::Display};

use thiserror::Error;

use super::parse_error::ParseError;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("failed to read `{0}`, {1}")]
    ReadFile(PathBuf, String),
    #[error("`{0}`:{1}")]
    Parse(PathBuf, ParseError),
	#[error("{0}")]
	Deserialize(String),
}

impl serde::de::Error for ReadFileError{
#[doc = " Raised when there is general error when deserializing a type."]
#[doc = ""]
#[doc = " The message should not be capitalized and should not end with a period."]
#[doc = ""]
#[doc = " ```edition2021"]
#[doc = " # use std::str::FromStr;"]
#[doc = " #"]
#[doc = " # struct IpAddr;"]
#[doc = " #"]
#[doc = " # impl FromStr for IpAddr {"]
#[doc = " #     type Err = String;"]
#[doc = " #"]
#[doc = " #     fn from_str(_: &str) -> Result<Self, String> {"]
#[doc = " #         unimplemented!()"]
#[doc = " #     }"]
#[doc = " # }"]
#[doc = " #"]
#[doc = " use serde::de::{self, Deserialize, Deserializer};"]
#[doc = ""]
#[doc = " impl<\\'de> Deserialize<\\'de> for IpAddr {"]
#[doc = "     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>"]
#[doc = "     where"]
#[doc = "         D: Deserializer<\\'de>,"]
#[doc = "     {"]
#[doc = "         let s = String::deserialize(deserializer)?;"]
#[doc = "         s.parse().map_err(de::Error::custom)"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
fn custom<T>(msg:T) -> Self where T:Display {
    Self::Deserialize(format!("{}", msg))
}
}