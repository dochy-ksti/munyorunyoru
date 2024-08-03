use crate::samples::html_samples::html_builder::{HtmlItem, Param, Tag};
use munyo::RestOf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Tags {
    Text(RestOf),
    BR,
    H3(String, Vec<Tags>),
    P(Vec<Tags>),
    Blockquote(Vec<Tags>),
}

pub fn to_html_items(items: &[Tags]) -> Vec<HtmlItem> {
    let mut r: Vec<HtmlItem> = vec![];
    for item in items {
        match item {
            Tags::Text(t) =>{
                r.push(text(&t.arg));
            }
            Tags::BR =>{
                r.push(tag("br", vec![], vec![]))
            }
            Tags::H3(c, tags) => {
                r.push(tag("h3", class(c), to_html_items(tags)));
            }
            Tags::P(tags) => {
                r.push(tag("p", vec![], to_html_items(tags)));
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

fn class(class: &str) -> Vec<Param> {
    vec![Param {
        name: "class".to_string(),
        value: class.to_string(),
    }]
}

