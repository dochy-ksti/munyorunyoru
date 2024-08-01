use crate::samples::html_samples::html_builder::{HtmlItem, Param, Tag};
use munyo::RestOf;
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

