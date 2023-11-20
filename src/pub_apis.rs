use std::path::Path;

use crate::{MunyoDeserializer, MunyoSerializer, Result};
use serde::{de, ser::Serialize};

/// Deserialize items from the text of the Munyo language
pub fn from_str<T>(text: &str) -> Result<Vec<T>>
where
    T: de::DeserializeOwned
{
    let mut de = MunyoDeserializer::new(text)?;
    de::Deserialize::deserialize(&mut de)
}

/// Deserialize items from the path of the source file of the Munyo language
pub fn from_file<P, T>(path : P) -> Result<Vec<T>>
	where P : AsRef<Path>, T : de::DeserializeOwned
{
	let s = std::fs::read_to_string(&path).map_err(|e| 
		crate::Error::ReadFile(path.as_ref().to_path_buf(), format!("{}", e)))?;
	let mut de = MunyoDeserializer::with_path(&s, path.as_ref().to_path_buf())?;
    de::Deserialize::deserialize(&mut de)
}

/// Deserialize items from the text and the path of the source file of the Munyo language
/// 
/// The path is only used for the error messages
pub fn from_str_with_path<P,T>(text : &str, path : P) -> Result<Vec<T>>
	where P : AsRef<Path>, T : de::DeserializeOwned
{
	let mut de = MunyoDeserializer::with_path(text, path.as_ref().to_path_buf())?;
    de::Deserialize::deserialize(&mut de)
}

/// Serialize items to the text of Munyo language
pub fn to_string<T>(items: &[T]) -> Result<String>
where
    T: Serialize,
{
    let mut ser = MunyoSerializer::new();
    items.serialize(&mut ser)?;
    Ok(ser.into_string())
}
