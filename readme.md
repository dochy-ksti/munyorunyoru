# Munyo
[![crates.io link](https://img.shields.io/crates/v/munyo.svg)](https://crates.io/crates/munyo)
[![Doc link](https://docs.rs/munyo/badge.svg)](https://docs.rs/munyo)
### Munyo

Munyo is a data language which aims to be the most efficient way to handwrite data. You can also see a clear error message along with the line number when an error occurs.

For example, you can create [a domain-specific language with just a little coding](https://github.com/dochy-ksti/munyorunyoru/tree/master/src/samples/html_samples/sample3). 

This language can efficiently write HTML documents which describe conversations of Alice and Bob in a specific format.

## Generated HTML
![Screenshot of the HTML page created from the DSL.](dsl_sample.png)

The main purpose of this library is to be used for writing data that is too large or too complex to write in JSON/TOML/YAML/etc..., so imagine a conversation between Alice and Bob that goes on long enough to become 10k bytes of string. This sample couldn't be made that long.

If Alice and Bob’s conversation is that long, it would be worth creating a little language for it.

## Munyo Source File
```
H3 Domain Specific Sample|class ribbon1

Alice I’ve arrived in Honolulu.
Bob I’m on the Moon!
Alice Let’s observe quantum entanglement and confirm the violation of Bell’s inequality.
Bob Let’s do it!

Blockquote
    P God doesn't play dice
    || <cite> tag is more appropriate.
    P —Albert Einstein|class right
```
The Munyo language is basically:
```
Typename arg1 arg2...|param_name1 param_value1|param_name2 param_value2...
    Typename arg1...  <-Indentation means the parent line contains the indented lines as children.
```
A line is statically typed, and each line needs a backing Rust data structure which is enum variant to deserialize with Serde.

## Rust Code
```Rust
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::from_file;
    use crate::samples::html_samples::sample3::tags::{to_html_items, Item};

    let path = "src/samples/html_samples/sample3/sample3.munyo";
    // deserialize Munyo file as Items
    let v: Vec<Item> = from_file(path)?;
    // convert Items to HTML
    let b = HtmlBuilder {
        items: to_html_items(&v),
        title: "Sample3".to_string(),
        stylesheet: Some("sample.css".to_string()),
        ..Default::default()
    };
    let output = b.to_string();
    std::fs::write("src/samples/html_samples/sample3/output.html", output).unwrap();
    Ok(())
}

use crate::{
    samples::html_samples::html_builder::{HtmlItem, Param, Tag},
    RestOf,
};
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
```
You can define your language with Munyo and backing Rust code. You should make the language
as efficient as possible for the data you want to write.

## What is DSL?

The term "DSL" in this document may confuse you. [Here](https://github.com/dochy-ksti/munyorunyoru/blob/master/whats_dsl.md) is the explanation.

## Motivation

The motivation is explained [here](https://github.com/dochy-ksti/munyorunyoru/blob/master/motivation.md)

## Direct Conversion

This document is already too long for a readme, but there are still many things that have not been explained enough. Please read if you don’t mind.

The above is the most general usage of this library, but sometimes you can get your work done without converting Munyo to your own Rust data structure. [Converting to HTML is one of them](https://github.com/dochy-ksti/munyorunyoru/tree/master/src/samples/html_samples/sample4). 

Munyo language can be naturally converted to HTML, and in this case, you don’t need to create enum variant for each HTML tag.

## Munyo Source File To Be Directly Converted To HTML
```
|| Set default type to "Text". 
|| In this case it makes this example more redundant, 
|| but this functionality must be shown somewhere.
|| See "lang_spec.txt" for details.
>>Text

|| ">\" means canceling default type, 
|| so the type of this line is "h3"
>\h3 Domain Specific Sample|class ribbon1

>\div|class balloon balloonL
    >\div|class balloon-img
        >\figure
            >\img|src girl.png
            >\figcaption Alice
    >\div|class balloon-text
        >\div|class balloon-text-inner

            || This line doesn't have a type, 
            || so the default type "Text" is applied.
            I've arrived in Honolulu.

>\div|class balloon balloonR
    >\div|class balloon-img
        >\figure
            >\img|src boy.png
            >\figcaption Bob
    >\div|class balloon-text
        >\div|class balloon-text-inner

            || This is "Text" too
            I'm on the Moon!
            ...
```
It seems that writing a source file in Munyo language to be directly converted to HTML is not a good idea.

When we give "Alice" and "Bob" special treatment, the redundancy will be greatly reduced.

## Specialized Munyo File To Be Directly Converted To HTML
```
h3 Domain Specific Sample|class ribbon1

Alice I've arrived in Honolulu.
Bob I'm on the Moon!
Alice Let’s observe quantum entanglement and confirm the violation of Bell’s inequality.
Bob Let’s do it!

blockquote
    p GOD DOES NOT PLAY DICE.
    cite —Albert Einstein
```
It seems that only the first capital letter of the tags has been changed to lowercase from the statically typed example, but this time, it was possible to create the "cite" tag without coding. In addition, the code has been simplified as follows.

## Converting Munyo to HTML with untyped MunyoValue
```Rust
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::samples::html_samples::sample4::untyped::to_html_items;
    use crate::MunyoItem;

    let path = "src/samples/html_samples/sample4/untyped.munyo";
    // deserialize a Munyo file as MunyoItems.
    // MunyoItem is the untyped data type of the Munyo language,
    // like serde_json::value::Value
    let v = MunyoItem::from_file(path)?;
    let b = HtmlBuilder {
        items: to_html_items(&v)?,
        title: "untyped sample".to_string(),
        stylesheet: Some("sample.css".to_string()),
        ..Default::default()
    };
    let output = b.to_string();
    std::fs::write("src/samples/html_samples/sample4/output.html", output).unwrap();
    Ok(())
}

use crate::{
    samples::html_samples::html_builder::{HtmlItem, Param, Tag},
    MunyoItem,
};
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
    let t = format!(r###"
<div class="balloon {bl}">
  <div class="balloon-img"><figure><img src="{pict}" /><figcaption>{speaker}</figcaption></figure></div>
  <div class="balloon-text"><div class="balloon-text-inner">
    {text}
  </div></div>
</div>"###);
    HtmlItem::Text(t)
}

fn tag(
    tag_name: &str,
    argument: &str,
    params: &BTreeMap<String, String>,
    children: &[MunyoItem],
) -> crate::Result<HtmlItem> {
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
```
There are still many things to be explained. Please read the [doc](https://docs.rs/munyo) for details.

## Async

This crate also contains the [concurrent](https://docs.rs/munyo/latest/munyo/struct.Concurrent.html) version of the functions to deserialize, and runtime agnostic async fn to receive the deserialized data concurrently.

## Usage

Add these to your `cargo.toml`:

```
[dependencies]
munyo = "0.3"
serde = { version = "1", features = ["derive"] }
```
## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.