use std::path::{PathBuf, Path};

use crate::{MunyoDeserializer, MunyoSerializer, Result};
use serde::{de, ser::Serialize};
pub fn from_str<'de, T>(text: &'de str) -> Result<Vec<T>>
where
    T: de::Deserialize<'de>,
{
    let mut de = MunyoDeserializer::new(text, None)?;
    de::Deserialize::deserialize(&mut de)
}

pub fn from_file<'de, P, T>(path : P) -> Result<Vec<T>>
	where P : AsRef<Path>, T : de::DeserializeOwned
{
	let s = std::fs::read_to_string(path.as_ref()).map_err(|e| 
		crate::Error::ReadFile(path.as_ref().to_path_buf(), format!("{}", e)))?;
	from_str(&s)
}

pub fn to_string<T>(items: &[T]) -> Result<String>
where
    T: Serialize,
{
    let mut ser = MunyoSerializer::new();
    items.serialize(&mut ser)?;
    Ok(ser.into_string())
}
