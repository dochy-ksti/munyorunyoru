use std::path::Path;

use crate::{MunyoDeserializer, MunyoSerializer, Result};
use serde::{de, ser::Serialize};

/// Deserialize items from the text of the Munyo language.
/// 
/// # Example
/// ```
/// #[derive(serde::Deserialize, Debug, PartialEq)]
/// enum Enum{
/// 	Foo,
/// }
/// fn main() -> munyo::Result<()>{
/// 	let v : Vec<Enum> = munyo::from_str("Foo")?;
/// 	assert_eq!(&v[0], &Enum::Foo);
/// 	Ok(())
/// }
/// ```
pub fn from_str<T>(text: &str) -> Result<Vec<T>>
where
    T: de::DeserializeOwned
{
    let mut de = MunyoDeserializer::new(text)?;
    de::Deserialize::deserialize(&mut de)
}

/// Deserialize items from the path of the source file of the Munyo language.
/// 
/// # Example
/// ```
/// #[derive(serde::Deserialize, Debug, PartialEq)]
/// enum Enum{
/// 	Foo,
/// }
/// fn main() -> munyo::Result<()>{
/// 	let text = "Foo";
/// 	// Write the text to a file and get the path.
/// 	# let file = munyo::temp(text)?;
/// 	# let path = file.path();
/// 	let v : Vec<Enum> = munyo::from_file(path)?;
/// 	assert_eq!(&v[0], &Enum::Foo);
/// 	Ok(())
/// }
/// ```
pub fn from_file<P, T>(path : P) -> Result<Vec<T>>
	where P : AsRef<Path>, T : de::DeserializeOwned
{
	let s = crate::read_file(&path)?;
	let mut de = MunyoDeserializer::with_path(&s, path.as_ref())?;
    de::Deserialize::deserialize(&mut de)
}

/// Deserialize items from the text and the path of the source file of the Munyo language
/// 
/// The path is only used for the error messages. See [from_str] for the usage.
pub fn from_str_with_path<P,T>(text : &str, path : P) -> Result<Vec<T>>
	where P : AsRef<Path>, T : de::DeserializeOwned
{
	let mut de = MunyoDeserializer::with_path(text, path.as_ref())?;
    de::Deserialize::deserialize(&mut de)
}

/// Serialize items to the text of Munyo language
/// 
/// # Example
/// ```
/// #[derive(serde::Serialize)]
/// enum Enum{
///     Item(usize, String, f64, Vec<Enum>)
/// }
/// fn main() -> munyo::Result<()>{
///     let item = Enum::Item(5, "s".to_string(), 1.1, vec![
///         Enum::Item(4,"t".to_string(), 3.4, vec![])
///     ]);
///     let items = vec![item];
///     let text = munyo::to_string(&items)?;
///     assert_eq!(&text, 
/// r"Item 5 s 1.1
/// 	Item 4 t 3.4
/// ");
///     Ok(())
/// }
/// ```
pub fn to_string<T>(items: &[T]) -> Result<String>
where
	T: Serialize,
{
    let mut ser = MunyoSerializer::new();
    items.serialize(&mut ser)?;
    Ok(ser.into_string())
}
