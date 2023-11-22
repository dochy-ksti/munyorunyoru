use crate::{
    samples::html_samples::html_builder::{HtmlItem, Param, Tag},
    RestOf,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Tags {
    H3(RestOf, Class),
    P(RestOf),
    Blockquote(Vec<Tags>),
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: String,
}

pub fn to_html_items(items: &[Tags]) -> Vec<HtmlItem> {
    let mut r: Vec<HtmlItem> = vec![];
    for item in items {
        match item {
            Tags::H3(t, c) => {
                r.push(tag("h3", class(c), vec![text(&t.arg)]));
            }
            Tags::P(t) => {
                r.push(tag("p", vec![], vec![text(&t.arg)]));
            }
            Tags::Blockquote(v) => r.push(tag("blockquote", vec![], to_html_items(v))),
        }
    }
    r
}

fn text(s: &str) -> HtmlItem {
    HtmlItem::Text(s.to_string())
}

fn tag(name: &str, params: Vec<Param>, children: Vec<HtmlItem>) -> HtmlItem {
    HtmlItem::Tag(
        Tag {
            name: name.to_string(),
            params,
        },
        children,
    )
}

fn class(class: &Class) -> Vec<Param> {
    vec![Param {
        name: "class".to_string(),
        value: class.class.clone(),
    }]
}

#[test]
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::from_file;
	
    let v: Vec<Tags> = from_file("src/samples/html_samples/sample1/sample1.munyo")?;
    let b = HtmlBuilder {
        items: to_html_items(&v),
        title: "Sample1".to_string(),
        stylesheet: Some("sample.css".to_string()),
        ..Default::default()
    };
    let output = b.to_string();
    std::fs::write("src/samples/html_samples/sample1/output.html", output).unwrap();
    Ok(())
}
