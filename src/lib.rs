#![allow(clippy::module_inception)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

//! Munyo is a data language. The goal of this language is to be the most efficient way to handwrite data.
//!
//! All the things to use Munyo are explained here.
//!
//! Munyo language looks like this:
//! ```text
//! Typename arg1 arg2|param_name1 value1|param_name2 value2
//! ```
//! The start of the line is typename, and it's followed by args. They are delimited by space.
//! If the type has params, the line is followed by "|param_name value"s.
//!
//! Here is the Rust code corresponds to this.
//! ```
//! use serde::{Serialize, Deserialize};
//!
//! // Munyo basically needs `serde` to (de)serialize.
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! // A line is deserialized to one of the enum variant.
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
//! "Typename args_are_delimited_by_spaces 32|param_name1 2.1|param_name2 params string is not delimited";
//! 	let items : Vec<EnumName> = munyo::from_str(munyo_text)?;
//! 	assert_eq!(items[0], EnumName::Typename(
//! 		"args_are_delimited_by_spaces".to_string(),
//! 		32,
//! 		ParamStruct{
//! 			param_name1 : 2.1,
//! 			param_name2 : "params string is not delimited".to_string()
//! 		}
//! 	));
//! 	Ok(())
//! }
//! ```
//! As described above, args are delimited by spaces,
//! but param value is not delimited(param_name is delimited from param_value by space, just in case.)
//!
//! You can use RestOf to ignore the spaces in the argument.
//! ```
//! use serde::Deserialize;
//! // derive(Serialize) is not mandatory.
//! #[derive(Deserialize, PartialEq, Debug)]
//! enum EnumName2{
//! 	Typename(usize, munyo::RestOf)
//! }
//! fn main() -> munyo::Result<()>{
//! 	let items : Vec<EnumName2> = munyo::from_str(
//! 		"Typename 10 RestOf can ignore spaces in the arg")?;
//! 	assert_eq!(items[0], EnumName2::Typename(10, munyo::RestOf::new(
//! 		"RestOf can ignore spaces in the arg".to_string())));
//! 	let items : Vec<EnumName2> = munyo::from_str(
//! 		"Typename 30  Preceding spaces and spaces before the end of the line are also captured in RestOf   ")?;
//! 	assert_eq!(items[0], EnumName2::Typename(30, munyo::RestOf::new(
//! 		" Preceding spaces and spaces before the end of the line are also captured in RestOf   ".to_string())));
//! 	Ok(())
//! }
//! ```
//! Munyo can't (de)serialize arbitrary rust data structures. It can only (de)serializes Vec of enum.
//! A line of Munyo text is deserialized to one of the enum variant, but StructVariant is not supported.
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
//! All values must implement Serialize/Deserialize trait of `serde` to serialize/deserialize.
//! You can easily do that with #[derive(Serialize, Deserialize)].
//! You can also implement them by yourself.[example1][example2]
//! ```
//! use serde::Deserialize;
//! #[derive(Deserialize, Debug, PartialEq)]
//! enum Enum{
//! 	Typename(munyo::samples::color::Color)
//! }
//! fn main() -> munyo::Result<()>{
//! 	// Color has an original implementation to deserialize the str "#xxx_xxx_xxx" to RGB.
//! 	let v : Vec<Enum> = munyo::from_str("Typename #120_220_10")?;
//! 	assert_eq!(v[0], Enum::Typename(munyo::samples::color::Color::new(120,220,10)));
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
//! #[derive(Deserialize, Debug, PartialEq)]
//! enum Parent{
//! 	//Only one children is allowed.
//! 	ParentItem1(String, Vec<Child>),
//! 	//Children's type can be the same as the parent.
//! 	ParentItem2(usize, Vec<Parent>)
//! }
//! #[derive(Deserialize, Debug, PartialEq)]
//! enum Child{
//! 	ChildItem1(StructsCanHaveOption),
//! 	ChildItem2
//! }
//!
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct StructsCanHaveOption{
//! 	name1 : Option<u32>
//! }
//! fn main() -> munyo::Result<()>{
//! 	let v : Vec<Parent> = munyo::from_str(r###"
//! ParentItem1 aaa
//! 	ChildItem1|  name1 100|| Preceding spaces before param names are ignored.
//! 	ChildItem2
//! ParentItem2 20
//! 	ParentItem1|| String arguments can be empty
//! 		ChildItem1|| param of Option can be omitted
//! 	ParentItem2 10|| Non-string arguments can't be empty.
//! 		|| No children
//! "###)?;
//! 	assert_eq!(&v[0], &Parent::ParentItem1("aaa".to_string(), vec![
//! 		Child::ChildItem1(StructsCanHaveOption{ name1 : Some(100) }),
//! 		Child::ChildItem2]));
//! 	assert_eq!(&v[1], &Parent::ParentItem2(20, vec![
//! 		Parent::ParentItem1("".to_string(), vec![
//! 			Child::ChildItem1(StructsCanHaveOption{ name1 : None })
//! 		]),
//! 		Parent::ParentItem2(10, vec![])
//! 	]));
//! 	Ok(())
//! }
//! ```
//! As described above, String arguments can be empty, and from '||' to the end of the line
//! is considered as a comment in Munyo. Fields of structs can be Option, but args can't.
//!
//! In Munyo, '\\' and '|' are special characters, so you need to escape them as '\\\\', '\\|' to
//! write them as normal characters. Also, '\t','\r','\n' can be used for tab, carriage-return,
//! and newline.
//!
//! There are three types of line continuations in Munyo:
//! ```
//! #[derive(serde::Deserialize, Debug,PartialEq)]
//! enum Enum{
//! 	Foo(munyo::RestOf, Struct)
//! }
//! #[derive(serde::Deserialize, Debug,PartialEq)]
//! struct Struct{
//! 	param : String
//! }
//! fn main() -> munyo::Result<()>{
//! 	let text = r###"
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
//! "###;
//! 	let v : Vec<Enum> = munyo::from_str(text)?;
//! 	fn get(foo : &Enum) -> &str{
//! 		match foo{
//! 			Enum::Foo(rest_of, _) => &rest_of.arg
//! 		}
//! 	}
//! 	assert_eq!(get(&v[0]), "This line is continued to the next line. The next line can have arbitrary number of tabs at the start. The tabs are ignored.");
//! 	assert_eq!(get(&v[1]), r###"If you put '\' at the end of the line,
//! the line is continued to the next line,
//! and those lines are separated with line-break code used in the text."###);
//! 	assert_eq!(get(&v[2]), "In this case, arg can't be continued. Only params can be attached in the next line.");
//! 	let Enum::Foo(_,s) = &v[2];
//! 	assert_eq!(&s.param, "value");
//!
//! 	Ok(())
//! }
//! ```
//! An empty line or a line which only has a comment or tabs is ignored.
//!
//! If you want to write line continuation with comments, you can use '|||' and '||\\'. Check the
//! [language specifications]() for details.
//!
//! You can use [MunyoItem] to serialize/deserialize Munyo without serde.
//! ```
//! fn main() -> munyo::Result<()>{
//! 	let v = munyo::MunyoItem::from_str("Typename  argu ment  |   param  value ")?.result;
//! 	assert_eq!(&v[0].typename, "Typename");
//! 	// One space works as a delimiter. Other spaces are recognised as characters.
//! 	assert_eq!(&v[0].argument, " argu ment  ");
//! 	// Preceding spaces of param names are ignored. Spaces before the end of the line are recognised as characters.
//! 	assert_eq!(v[0].params.get("param").unwrap(), " value ");
//! 	Ok(())
//! }
//! ```
//! A line can have any number of args and structs, and zero or one children.
//! On the other hand, a line can't have two params with the same name.
//! ```text
//! || Multiple "param1" not allowed
//! Foo|param1 value|param1 value2
//! ```
//! On the other hand, a line with multiple structs which have the fields with the same name is allowed.
//! Those fields capture the same value.
//! ```
//! use serde::Deserialize;
//! #[derive(Deserialize, Debug, PartialEq)]
//! enum Enum{
//!     Foo(S1, S2)
//! }
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct S1{
//!     param_name1 : String,
//! }
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct S2{
//!     param_name1 : usize,
//! }
//! fn main() -> munyo::Result<()>{
//!     let v : Vec<Enum> = munyo::from_str("Foo|param_name1 20")?;
//!     assert_eq!(&v[0], &Enum::Foo(S1{param_name1 : "20".to_string()}, S2{param_name1 : 20 }));
//!     Ok(())
//! }
//! ```
//! I think that's all.

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
        .map_err(|e| crate::Error::ReadFile(path.as_ref().to_path_buf(), e.into()))
}

#[doc(hidden)]
/// This is only meant for testing.
///
/// The created file survives until the NamedTempFile drops
pub fn temp(s: &str) -> std::io::Result<tempfile::NamedTempFile> {
    use std::io::{Seek, SeekFrom, Write};
    let mut t = tempfile::NamedTempFile::new()?;
    writeln!(t, "{s}")?;
    t.seek(SeekFrom::Start(0))?;
    Ok(t)
}
