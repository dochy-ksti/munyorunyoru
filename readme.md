Munyo is a data language. You can create a domain-specific language with just a little coding.

```
H3 Domain Specific Sample|class ribbon1

Alice I’ve arrived in Honolulu.
Bob I’m on the Moon!
Alice Let’s observe quantum entanglement and confirm the violation of Bell’s inequality.
Bob Let’s do it!
```
↑ Munyo source file.

![Screenshot of the HTML page created from the DSL.](dsl_sample.png)

↑ HTML created from the source file.

```Rust
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
}

// This struct captures the parameter "class"
#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: String,
}

pub fn to_html_items(items: &[Item]) -> Vec<HtmlItem> {
    let mut r: Vec<HtmlItem> = vec![];
    for item in items {
        match item {
            Item::Alice(t) => {
                balloon(true, &t.arg, &mut r);
            }
            Item::Bob(t) => {
                balloon(false, &t.arg, &mut r);
            }
            Item::H3(t, c) => {
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
    HtmlItem::Tag(Tag::new(name.to_string(), params), children)
}

fn text(s: &str) -> HtmlItem {
    HtmlItem::Text(s.to_string())
}

fn class(class: &Class) -> Vec<Param> {
    vec![Param::new("class".to_string(), class.class.clone())]
}

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
```

↑ The code to generate HTML

The goal of this language is to be the most efficient way to handwrite data.

This crate also has the concurrent versions of the functions for deserializing, and runtime agnostic async fn to get deserialized data concurrently.

Please read [doc](doc_address) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.