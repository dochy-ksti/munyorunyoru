use crate::samples::html_samples::html_builder::{HtmlItem, Param, Tag};
use munyo::RestOf;
use serde::{Deserialize, Serialize};

// This enum defines the syntax.
#[derive(Serialize, Deserialize)]
pub enum Item {
    // RestOf captures all the remaining string of the line except parameters.
    Alice(RestOf),
    Bob(RestOf),
    // struct captures parameters as fields.
    H3(RestOf, Class),

    /// Blockquote can contain children
    Blockquote(Vec<Item>),
    P(RestOf, Class),
}

// This struct captures the parameter "class"
#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: Option<String>,
}

pub fn to_html_items(items: &[Item]) -> Vec<HtmlItem> {
    let mut r: Vec<HtmlItem> = vec![];
    for item in items {
        match item {
            Item::Alice(t) => {
                r.push(balloon(true, &t.arg));
            }
            Item::Bob(t) => {
                r.push(balloon(false, &t.arg));
            }
            Item::H3(t, c) => {
                r.push(tag_with_text("h3", &t.arg, class(c)));
            }
            Item::P(t, c) => {
                r.push(tag_with_text("p", &t.arg, class(c)));
            }
            Item::Blockquote(vec) => {
				r.push(tag_with_children("blockquote", to_html_items(vec)))
			}
        }
    }
    r
}

fn balloon(is_l: bool, text: &str) -> HtmlItem {
    let bl = if is_l { "balloonL" } else { "balloonR" };
    let pict = if is_l { "girl.png" } else { "boy.png" };
    let speaker = if is_l { "Alice" } else { "Bob" };
    let t = format!(r###"
<div class="balloon {bl}">
  <div class="balloon-img"><figure><img src="{pict}" /><figcaption>{speaker}</figcaption></figure></div>
  <div class="balloon-text"><div class="balloon-text-inner">
    {text}
  </div></div>
</div>"###);
    HtmlItem::Text(t)
}

fn tag_with_text(name: &str, text : &str, params: Vec<Param>) -> HtmlItem {
    HtmlItem::Tag(Tag::new(name.to_string(), params), vec![HtmlItem::Text(text.to_string())])
}

fn class(class: &Class) -> Vec<Param> {
    if let Some(c) = &class.class {
        vec![Param::new("class".to_string(), c.to_string())]
    } else {
        vec![]
    }
}

fn tag_with_children(name: &str, children : Vec<HtmlItem>) -> HtmlItem{
	HtmlItem::Tag(Tag::new(name.to_string(), vec![]), children)
}