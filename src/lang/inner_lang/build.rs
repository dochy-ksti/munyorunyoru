use pest::Parser;

use crate::{
    builder::builder::{Builder, MetaBuilder},
    lang::{builder_tree::BuilderTree, parse_main_line::LineResult, state::State},
};

use super::{InnerLangParser, Pairs, Rule};

pub(crate) fn build<MB, B>(
    state: &mut State,
    tree: &mut BuilderTree<B>,
    r: LineResult,
    meta_builder: &MB,
    start_index: usize,
) -> Result<(), String>
where
    MB: MetaBuilder<Item = B>,
    B: Builder,
{
    let (def, _emp) = state.default_types();

    let (name, arg) = if def.is_empty() {
        let p = InnerLangParser::parse(Rule::content, &r.content).expect("unreachable");
        parse_content(p)
    } else {
        let full = format!("{def} {}", r.content);
        let p = InnerLangParser::parse(Rule::content, &full).expect("unreachable");
        parse_content(p)
    };

    let mut builder = meta_builder.build(name, arg)?;

    let params = r.params;

    for param in params {
        let p = InnerLangParser::parse(Rule::param, &param).expect("unreachable");

        let (name, val) = parse_param(p);
        builder.set_param(name, val)?;
    }

    tree.add(builder, state.indent_level(), start_index)
        .expect("unreachable");
    Ok(())
}

pub(crate) fn build_empty_line_item<MB, B>(
    state: &mut State,
    tree: &mut BuilderTree<B>,
    meta_builder: &MB,
    start_index: usize,
) -> Result<(), String>
where
    MB: MetaBuilder<Item = B>,
{
    let (_def, emp) = state.default_types();

    if emp.is_empty() {
        return Ok(());
    }

    let emp_command = build_empty_line_command(emp, state.indent_level());

    let p = InnerLangParser::parse(Rule::content, &emp_command).expect("unreachable");
    let (name, arg) = parse_content(p);

    let builder = meta_builder.build(name, arg)?;

    tree.add(builder, state.indent_level(), start_index)
        .expect("unreachable");
    Ok(())
}

//premature optimization.
fn build_empty_line_command(emp_default: &str, indent_level: usize) -> String {
    let mut r: Vec<u8> = Vec::with_capacity(indent_level + emp_default.len());
    r.extend(std::iter::repeat(b'\t').take(indent_level));
    r.extend_from_slice(emp_default.as_bytes());
    unsafe { String::from_utf8_unchecked(r) }
}

fn parse_content(pairs: Pairs) -> (String, String) {
    let mut name = String::new();
    let mut text = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::name => name = pair.as_str().to_string(),
            Rule::text => text = pair.as_str().to_string(),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    (name, text)
}

fn parse_param(pairs: Pairs) -> (String, String) {
    parse_content(pairs)
}
