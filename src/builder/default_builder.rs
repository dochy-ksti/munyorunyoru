use std::{
    collections::{BTreeMap, btree_map::Entry},
    fmt::{Debug, Display},
};

use super::builder::{Builder, MetaBuilder};

#[derive(Debug, Clone, Default)]
pub struct DefaultMetaBuilder;


impl MetaBuilder for DefaultMetaBuilder {
    type Item = DefaultBuilder;

    fn build(&self, typename: String, argument: String) -> Result<Self::Item, String> {
        Ok(DefaultBuilder::new(typename, argument))
    }
}

#[derive(Debug)]
pub struct DefaultBuilder {
    pub(crate) typename: String,
    pub(crate) content: String,
    pub(crate) params: BTreeMap<String, String>,
    pub(crate) children: Vec<DefaultItem>,
}

impl DefaultBuilder {
    pub fn new(typename: String, argument: String) -> Self {
        Self {
            typename,
            content: argument,
            params: BTreeMap::new(),
            children: vec![],
        }
    }
}

impl Builder for DefaultBuilder {
    type Item = DefaultItem;

    fn set_param(&mut self, param_name: String, argument: String) -> Result<(), String> {
		match self.params.entry(param_name){
			Entry::Occupied(e) =>{
				return Err(format!("'{}' is applied multiple times", e.key()));
			}
			Entry::Vacant(e) =>{ e.insert(argument); }
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
            content: self.content,
            params: self.params,
            children: self.children,
        })
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct DefaultItem {
    pub typename: String,
    pub content: String,
    pub params: BTreeMap<String, String>,
    pub children: Vec<DefaultItem>,
}

impl Debug for DefaultItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_item(self, 0, f)
    }
}

impl Display for DefaultItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_item(self, 0, f)
    }
}

fn write_item(
    item: &DefaultItem,
    indent_level: usize,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for _ in 0..indent_level {
        write!(f, "\t")?;
    }
    write!(f, "{}", item_format(&item.typename, &item.content))?;
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
