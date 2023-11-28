use std::{
    collections::{btree_map::Entry, BTreeMap},
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use crate::{error::munyo_error::PathItem, lang::processed::Processed, read_file};

use super::builder::{Builder, MetaBuilder};

/// A meta builder which builds DefaultBuilder
#[derive(Debug, Clone, Default)]
pub struct DefaultMetaBuilder;

impl MetaBuilder for DefaultMetaBuilder {
    type Item = DefaultBuilder;

    fn build(&self, typename: String, argument: String) -> Result<Self::Item, String> {
        Ok(DefaultBuilder::new(typename, argument))
    }
}

/// A builder which builds MunyoItem
#[derive(Debug)]
pub struct DefaultBuilder {
    pub(crate) typename: String,
    pub(crate) argument: String,
    pub(crate) params: BTreeMap<String, String>,
    pub(crate) children: Vec<MunyoItem>,
}

impl DefaultBuilder {
    /// Creates DefaultBuilder
    pub fn new(typename: String, argument: String) -> Self {
        Self {
            typename,
            argument,
            params: BTreeMap::new(),
            children: vec![],
        }
    }
}

impl Builder for DefaultBuilder {
    type Item = MunyoItem;

    fn set_param(&mut self, param_name: String, argument: String) -> Result<(), String> {
        match self.params.entry(param_name) {
            Entry::Occupied(e) => {
                return Err(format!("'{}' is applied multiple times", e.key()));
            }
            Entry::Vacant(e) => {
                e.insert(argument);
            }
        }
        Ok(())
    }

    fn set_child(&mut self, child: Self::Item) -> Result<(), String> {
        self.children.push(child);
        Ok(())
    }

    fn finish(self) -> Result<Self::Item, String> {
        Ok(Self::Item {
            typename: self.typename,
            argument: self.argument,
            params: self.params,
            children: self.children,
        })
    }
}

/// Untyped Munyo value which can be used without implementing Serialize/Deserialize
#[derive(Clone, Default, PartialEq)]
pub struct MunyoItem {
	/// The typename of the line of the Munyo language
    pub typename: String,
	/// The argument of the line of the Munyo language
    pub argument: String,
	/// The params of the line of the Munyo language
    pub params: BTreeMap<String, String>,
	/// The children of the line of the Munyo language
    pub children: Vec<MunyoItem>,
}

impl Debug for MunyoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_item(self, 0, f)
    }
}

impl Display for MunyoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_item(self, 0, f)
    }
}

fn write_item(
    item: &MunyoItem,
    indent_level: usize,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for _ in 0..indent_level {
        write!(f, "\t")?;
    }
    write!(f, "{}", item_format(&item.typename, &item.argument))?;
    for (key, val) in &item.params {
        write!(f, "|{}", item_format(key, val))?;
    }
    for child in &item.children {
        writeln!(f)?;
        write_item(child, indent_level + 1, f)?;
    }
    Ok(())
}

fn item_format(name: &str, val: &str) -> String {
    if val.is_empty() {
        name.to_string()
    } else {
        format!("{} {}", name, val)
    }
}

impl MunyoItem {
    /// Deserialize `MunyoItem` from `&str` of the Munyo language.
    /// `path` is only used for error messages.
    ///
    /// See [from_str](Self::from_str) for details;
    pub fn from_str_with_path<P: AsRef<Path>>(
        s: &str,
        path: P,
    ) -> crate::Result<Processed<MunyoItem>> {
        Self::inner(s, Some(path.as_ref().to_path_buf()))
    }

    /// Deserialize `MunyoItem` from `&str` of the Munyo language.
    /// # Example
    /// ```
    /// # fn main() -> munyo::Result<()>{
    /// 	let v = munyo::MunyoItem::from_str("Typename  argu ment  |   param  value ")?.result;
    /// 	assert_eq!(&v[0].typename, "Typename");
    /// 	assert_eq!(&v[0].argument, " argu ment  ");
    /// 	assert_eq!(v[0].params.get("param").unwrap(), " value ");
    /// # 	Ok(())
    /// # }
    /// ```
    pub fn from_str(s: &str) -> crate::Result<Processed<MunyoItem>> {
        Self::inner(s, None)
    }

    /// Deserialize `MunyoItem` from `path` of the source file of the Munyo language.
    /// # Example
    /// ```
    /// # fn main() -> munyo::Result<()>{
    /// let s = "foo";
    /// // create a file with text "foo" and get the path
    /// # 	let file = munyo::temp(s)?;
    /// # 	let path = file.path();
    /// let v = munyo::MunyoItem::from_file(path)?.result;
    /// assert_eq!(&v[0].typename, "foo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Processed<MunyoItem>> {
        let buf = path.as_ref().to_path_buf();
        let s = read_file(&buf)?;
        Self::inner(&s, Some(buf))
    }

    fn inner(s: &str, path: Option<PathBuf>) -> crate::Result<Processed<MunyoItem>> {
        crate::from_str_with_metabuilder(s, &DefaultMetaBuilder)
            .map_err(|e| crate::Error::Parse(PathItem::new(path), e))
    }
}
