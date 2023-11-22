#[test]
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::from_file;
    use crate::samples::html_samples::sample2::tags::{to_html_items, Tags};

    let v: Vec<Tags> = from_file("src/samples/html_samples/sample2/sample2.munyo")?;
    let b = HtmlBuilder {
        items: to_html_items(&v),
        title: "Sample2".to_string(),
        stylesheet: Some("sample.css".to_string()),
        ..Default::default()
    };
    let output = b.to_string();
    std::fs::write("src/samples/html_samples/sample2/output.html", output).unwrap();
    Ok(())
}
