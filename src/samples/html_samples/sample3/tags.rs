use crate::{
    samples::html_samples::html_builder::{HtmlItem, Param, Tag},
    RestOf,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Tags {
    Alice(RestOf),
    Bob(RestOf),
    H3(RestOf, Class),
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: String,
}

pub fn to_html_items(items: &[Tags]) -> Vec<HtmlItem> {
    let mut r: Vec<HtmlItem> = vec![];
    for item in items {
        match item {
            Tags::Alice(t) => {
                balloon(true, &t.arg, &mut r);
            }
            Tags::Bob(t) => {
                balloon(false, &t.arg, &mut r);
            }
            Tags::H3(t, c) => {
                r.push(tag("h3", class(c), vec![text(&t.arg)]));
            }
        }
    }
    r
}

fn balloon(is_l: bool, text: &str, r: &mut Vec<HtmlItem>) {
    let bl = if is_l { "balloonL" } else { "balloonR" };
    let pict = if is_l { "girl.png" } else { "boy.png" };
    let speaker = if is_l { "Alice" } else { "Bob" };
    let t = format!(
        r###"
<div class="balloon {}">
  <div class="balloon-img"><figure><img src="{}" /><figcaption>{}</figcaption></figure></div>
  <div class="balloon-text"><div class="balloon-text-inner">
  {}
  </div></div>
</div>"###,
        bl, pict, speaker, text
    );
    r.push(self::text(&t))
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

fn text(s: &str) -> HtmlItem {
    HtmlItem::Text(s.to_string())
}

fn class(class: &Class) -> Vec<Param> {
    vec![Param {
        name: "class".to_string(),
        value: class.class.clone(),
    }]
}
