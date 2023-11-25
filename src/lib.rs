#![allow(clippy::module_inception)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

//! Munyo is a data language. The goal of this language is to be the most efficient way to handwrite data.
//!
//! All the basics to use Munyo are explained here.
//!
//! Munyo language looks like this:
//! ```text
//! Typename arg1 arg2|param_name1 value1|param_name2 value2
//! ```
//! The start of the line is typename, and it's followed by args. They are delimited by space.
//! If the type has params, the line is followed by "|param_name value".
//!
//! Here is the Rust code corresponds to this.
//! ```
//! use serde::{Serialize, Deserialize};
//!
//! // Munyo needs `serde` to (de)serialize.
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! // A line is (de)serialized to one of the enum variant.
//! enum EnumName{
//! 	Typename(String, usize, ParamStruct)
//! }
//!
//! // Structs capture params as fields
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct ParamStruct{
//! 	param_name1 : f64,
//! 	param_name2 : String,
//! }
//! //Let's deserialize.
//! fn main() -> munyo::Result<()>{
//! 	let munyo_text =
//! "Typename args_are_delimited_by_space 32|param_name1 2.1|param_name2 params string is not delimited";
//! 	let items : Vec<EnumName> = munyo::from_str(munyo_text)?;
//! 	assert_eq!(items[0], EnumName::Typename(
//! 		"args_are_delimited_by_space".to_string(),
//! 		32,
//! 		ParamStruct{
//! 			param_name1 : 2.1,
//! 			param_name2 : "params string is not delimited".to_string()
//! 		}
//! 	));
//! 	Ok(())
//! }
//! ```
//! As described above, args are delimited by space,
//! but param value is not delimited(param_name is delimited from param_value by space, just in case.)
//!
//! You can use RestOf to ignore the delimiters(space) in the argument.
//! ```
//! use serde::Deserialize;
//! // derive(Serialize) is not mandatory.
//! #[derive(Deserialize, PartialEq, Debug)]
//! enum EnumName2{
//! 	Variant(usize, munyo::RestOf)
//! }
//! fn main() -> munyo::Result<()>{
//! 	let items : Vec<EnumName2> = munyo::from_str(
//! 		"Variant 10 RestOf can ignore spaces in the arg")?;
//! 	assert_eq!(items[0], EnumName2::Variant(10, munyo::RestOf::new(
//! 		"RestOf can ignore spaces in the arg".to_string())));
//! 	Ok(())
//! }
//! ```
//! Munyo can't (de)serialize arbitrary rust data structures. It can only (de)serializes Vec of enum.
//! A line of Munyo text is (de)serialized to one of the enum variant, but StructVariant is not supported.
//! ```
//! enum Enum{
//! 	// Not supported
//! 	StructVariant{ name : String, name2 : usize },
//! 	// supported
//! 	TupleVariant(String, usize),
//! 	// supported
//! 	UnitVariant
//! }
//! ```
//! All values must implement Serialize/Deserialize trait to serialize/deserialize.
//! You can easily do that with #[derive(Serialize, Deserialize)].
//! You can also implement them by yourself.[example1][example2]
//! ```
//! use serde::Deserialize;
//! #[derive(Deserialize, Debug, PartialEq)]
//! enum Enum{
//! 	Variant(munyo::samples::color::Color)
//! }
//! fn main() -> munyo::Result<()>{
//! 	// Color has an original implementation to deserialize #xxx_xxx_xxx to RGB values.
//! 	let v : Vec<Enum> = munyo::from_str("Variant #120_220_10")?;
//! 	assert_eq!(v[0], Enum::Variant(munyo::samples::color::Color::new(120,220,10)));
//! 	Ok(())
//! }
//! ```
//! Munyo is an indent-based language to represent tree structure.
//! ```text
//! ParentItem1 arg
//! 	ChildItem1 arg
//! 	ChildItem2
//! ```
//! Tabs must be used for indents. Some text editors automatically convert tabs to spaces, so please be aware.
//!
//! A line can have only one children, and it's also `Vec<enum>`.
//! ```
//! use serde::Deserialize;
//! #[derive(Deserialize)]
//! enum Parent{
//! 	//Only one children is allowed.
//! 	ParentItem1(String, Vec<Child>),
//! 	//Children's type can be the same as the parent.
//! 	ParentItem2(usize, Vec<Parent>)
//! }
//! #[derive(Deserialize)]
//! enum Child{
//! 	ChildItem1(StructsCanHaveOption),
//! 	ChildItem2
//! }
//!
//! #[derive(Deserialize)]
//! struct StructsCanHaveOption{
//! 	name1 : Option<u32>
//! }
//! fn main() -> munyo::Result<()>{
//! 	let v : Vec<Parent> = munyo::from_str(r###"
//! ParentItem1 aaa
//! 	ChildItem1|name1 100
//! 	ChildItem2
//! ParentItem2 20
//! 	ParentItem1|| String argument can be empty
//! 		ChildItem1|| param of Option can be omitted
//! 	ParentItem2 10|| Non-string arguments can't be empty.
//! 		|| No children
//! "###)?;
//! 	Ok(())
//! }
//! ```
//! As described above, String argument can be empty, and from '||' to the end of the line
//! is considered as a comment in Munyo. Struct's fields can be Option, but args can't.
//!
//! In Munyo, '\\' and '|' are special characters, so you need to escape them as '\\\\', '\\|' to
//! write them as normal characters. Also, '\t','\r','\n' can be used for tab, carriage-return,
//! and new line.
//!
//! There are three types of line continuations in Munyo:
//! ```text
//! || '|' at the end of the line
//! Foo This line is continued |
//! 		to the next line. The next line can have arbitrary number |
//! 	of tabs at the start. |
//! The tabs are ignored.|param value
//!
//! || '\' at the end of the line
//! Foo If you put '\\' at the end of the line,\
//! 	the line is continued to the next line,\
//! and those lines are separated with line-break code used in the text.|param value
//!
//! || '|' at the start of the next line
//! Foo In this case, arg can't be continued. Only params can be attached in the next line.
//! 		|param value
//! ```
//! If you want to write line continuation with comments, you can use '|||' and '||\\'. Check the
//! [language specification]() for details.
//!
//! You can use [MunyoItem] to serialize/deserialize Munyo without serde.
//! ```
//! fn main() -> munyo::Result<()>{
//! 	let v = munyo::MunyoItem::from_str("Foo 20|param value")?.result;
//! 	assert_eq!(&v[0].typename,"Foo");
//! 	assert_eq!(&v[0].argument,"20");
//! 	assert_eq!(v[0].params.get("param"), Some(&"value".to_string()));
//! 	Ok(())
//! }
//! ```
//!

pub mod builder;
pub mod error;
pub mod file_io;
pub mod lang;
mod pub_apis;
pub mod samples;
mod serde;
#[cfg(test)]
mod tests;

#[doc(inline)]
pub use crate::builder::default_builder::MunyoItem;
pub use crate::file_io::concurrent::Concurrent;
pub use crate::lang::from_str_with_metabuilder::from_str_with_metabuilder;
pub use crate::pub_apis::{from_file, from_str, from_str_with_path, to_string};
pub use crate::serde::deserializer::MunyoDeserializer;
pub use crate::serde::rest_of::RestOf;
pub use crate::serde::serializer::MunyoSerializer;

pub use crate::error::munyo_error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use std::path::Path;
pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> crate::Result<String> {
    std::fs::read_to_string(&path)
        .map_err(|e| crate::Error::ReadFile(path.as_ref().to_path_buf(), e.to_string()))
}
