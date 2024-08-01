#[test]
fn test() -> crate::Result<()> {
    use super::super::html_builder::HtmlBuilder;
    use crate::samples::html_samples::sample4::untyped::to_html_items;
    use munyo::MunyoItem;

    let path = "src/samples/html_samples/sample4/untyped.munyo";
    // deserialize Munyo file as MunyoItems.
    // MunyoItem is an untyped data structure of the Munyo language.
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
