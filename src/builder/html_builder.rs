// use std::{
//     collections::BTreeMap,
//     fmt::{Debug, Display},
// };

// use super::builder::{Builder, MetaBuilder};

// #[derive(Debug, Clone, Default)]
// pub struct HtmlMetaBuilder {}

// impl HtmlMetaBuilder {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// impl MetaBuilder for HtmlMetaBuilder {
//     type Item = HtmlBuilder;

//     fn build(&self, typename: String, argument: String) -> Result<Self::Item, String> {
//         Ok(HtmlBuilder::new(typename, argument))
//     }
// }

// #[derive(Debug)]
// pub struct HtmlBuilder {
//     typename: String,
//     content: String,
//     params: BTreeMap<String, String>,
//     children: Vec<HtmlItem>,
// }

// impl HtmlBuilder {
//     pub fn new(typename: String, argument: String) -> Self {
//         Self {
//             typename,
//             content: argument,
//             params: BTreeMap::new(),
//             children: vec![],
//         }
//     }
// }

// impl Builder for HtmlBuilder {
//     type Item = HtmlItem;

//     fn set_param(&mut self, param_name: String, argument: String) -> Result<(), String> {
//         let b = self.params.insert(param_name, argument);
//         if let Some(param_name) = b {
//             return Err(format!(
//                 "{param_name} is applied multiple times for {} {}",
//                 &self.typename, &self.content
//             ));
//         }
//         Ok(())
//     }

//     fn set_child(&mut self, child: Self::Item) -> Result<(), String> {
// 		if self.typename == "Text"{
// 			Err("Text can't have childs")?;
// 		}
//         self.children.push(child);
//         Ok(())
//     }

//     fn finish(self) -> Result<Self::Item, String> {
//         Ok(Self::Item {
//             typename: self.typename,
//             content: self.content,
//             params: self.params,
//             children: self.children,
//         })
//     }
// }

// #[derive(Clone, Default, PartialEq)]
// pub struct HtmlItem {
//     pub typename: String,
//     pub content: String,
//     pub params: BTreeMap<String, String>,
//     pub children: Vec<HtmlItem>,
// }

// impl Debug for HtmlItem {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write_item(self, 0, f)
//     }
// }

// impl Display for HtmlItem {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write_item(self, 0, f)
//     }
// }

// fn write_item(
//     item: &HtmlItem,
//     indent_level: usize,
//     f: &mut std::fmt::Formatter<'_>,
// ) -> std::fmt::Result {
//     for _ in 0..indent_level {
//         write!(f, "\t")?;
//     }
//     write!(f, "{}", item_format(&item.typename, &item.content))?;
//     for (key, val) in &item.params {
//         write!(f, "|{}", item_format(key, val))?;
//     }
//     for child in &item.children {
//         writeln!(f)?;
//         write_item(child, indent_level + 1, f)?;
//     }
//     Ok(())
// }

// fn item_format(name: &str, val: &str) -> String {
//     if val.is_empty() {
//         name.to_string()
//     } else {
//         format!("{} {}", name, val)
//     }
// }
