#[test]
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::from_file;
    use crate::samples::html_samples::sample3::tags::{to_html_items, Tags};
    //use crate::{from_str_with_metabuilder, DefaultMetaBuilder};

    let path = "src/samples/html_samples/sample3/sample3.munyo";
    let v: Vec<Tags> = from_file(path)?;
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
