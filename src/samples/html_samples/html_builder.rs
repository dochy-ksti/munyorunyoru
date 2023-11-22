#[derive(Debug)]
pub struct HtmlBuilder {
    pub items: Vec<HtmlItem>,
    pub lang: String,
    pub charset: String,
    pub title: String,
    pub stylesheet: Option<String>,
    pub script: Option<String>,
}

impl Default for HtmlBuilder {
    fn default() -> Self {
        fn s(s: &str) -> String {
            s.to_string()
        }

        Self {
            items: vec![],
            lang: s("en"),
            charset: s("utf-8"),
            title: s(""),
            stylesheet: None,
            script: None,
        }
    }
}

#[derive(Debug)]
pub enum HtmlItem {
    Text(String),
    Tag(Tag, Vec<HtmlItem>),
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub value: String,
}

impl HtmlBuilder {
    pub fn to_string(&self) -> String {
        format!(
            r###"
<!DOCTYPE html>
<html lang="{}">
<head>
  <meta charset="{}">
  <title>{}</title>
  {}
  {}
</head>
<body>
{}
</body>
</html>"###,
            &self.lang,
            &self.charset,
            &self.title,
            self.stylesheet
                .as_ref()
                .map_or(String::new(), |s| stylesheet(&s)),
            self.script.as_ref().map_or(String::new(), |s| script(s)),
            to_string(&self.items)
        )
    }
}

fn stylesheet(s: &str) -> String {
    format!(r###"<link rel="stylesheet" href="{}">"###, s)
}

fn script(s: &str) -> String {
    format!(r###"<script src="{}"></script>"###, s)
}

fn to_string(items: &[HtmlItem]) -> String {
    let mut r = String::new();
    inner(items, 0, &mut r);
    r
}

fn inner(items: &[HtmlItem], indent_level: usize, r: &mut String) {
    for item in items {
        match item {
            HtmlItem::Text(t) => {
                push(t, indent_level, r);
            }
            HtmlItem::Tag(t, vec) => {
                push(&t.to_string(), indent_level, r);
                inner(&vec, indent_level + 1, r);
                push(&t.closing_tag(), indent_level, r);
            }
        }
    }
}
fn push(s: &str, indent_level: usize, r: &mut String) {
    for _ in 0..indent_level {
        r.push('\t');
    }
    r.push_str(s);
    r.push('\n');
}

impl Tag {
    pub fn to_string(&self) -> String {
        let mut r = String::new();
        r.push('<');
        r.push_str(&self.name);

        for param in &self.params {
            r.push(' ');
            r.push_str(&param.name);
            r.push('=');
            r.push('"');
            r.push_str(&param.value);
            r.push('"');
        }
        r.push('>');
        r
    }

    pub fn closing_tag(&self) -> String {
        format!("</{}>", &self.name)
    }
}
