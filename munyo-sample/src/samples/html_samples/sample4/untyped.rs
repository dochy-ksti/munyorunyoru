use munyo::MunyoItem;

use crate::samples::html_samples::html_builder::{HtmlItem, Param, Tag};
use std::collections::BTreeMap;

pub fn to_html_items(items: &[MunyoItem]) -> crate::Result<Vec<HtmlItem>> {
    let mut vec: Vec<HtmlItem> = vec![];
    for item in items {
        match item.typename.as_str() {
            "Alice" => {
                if !item.children.is_empty() {
                    Err("Alice can't contain children")?
                }
                if !item.params.is_empty() {
                    Err("Alice can't contain params")?
                }
                vec.push(balloon(true, &item.argument));
            }
            "Bob" => {
                if !item.children.is_empty() {
                    Err("Bob can't contain children")?
                }
                if !item.params.is_empty() {
                    Err("Bob can't contain params")?
                }
                vec.push(balloon(false, &item.argument));
            }
            _ => vec.push(tag(
                &item.typename,
                &item.argument,
                &item.params,
                &item.children,
            )?),
        }
    }
    Ok(vec)
}

fn balloon(is_l: bool, text: &str) -> HtmlItem {
    let bl = if is_l { "balloonL" } else { "balloonR" };
    let pict = if is_l { "girl.png" } else { "boy.png" };
    let speaker = if is_l { "Alice" } else { "Bob" };
    let t = format!(
        r###"
<div class="balloon {bl}">
  <div class="balloon-img"><figure><img src="{pict}" /><figcaption>{speaker}</figcaption></figure></div>
  <div class="balloon-text"><div class="balloon-text-inner">
    {text}
  </div></div>
</div>"###
    );
    HtmlItem::Text(t)
}

fn tag(
    tag_name: &str,
    argument: &str,
    params: &BTreeMap<String, String>,
    children: &[MunyoItem],
) -> munyo::Result<HtmlItem> {
    let mut children = to_html_items(children)?;
    if !argument.is_empty() {
        // The argument will be the first child which is text.
        children.insert(0, HtmlItem::Text(argument.to_string()));
    }
    Ok(HtmlItem::Tag(
        Tag::new(
            tag_name.to_string(),
            params
                .iter()
                .map(|(name, value)| Param::new(name.to_string(), value.to_string()))
                .collect(),
        ),
        children,
    ))
}
