#[test]
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::from_file;
    use crate::samples::html_samples::sample3::tags::{to_html_items, Item};

    let path = "src/samples/html_samples/sample3/sample3.munyo";
    // deserialize Munyo file as Items
    let v: Vec<Item> = from_file(path)?;
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
