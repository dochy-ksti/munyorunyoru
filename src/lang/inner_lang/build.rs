use anyhow::{anyhow, Error};
use pest::Parser;

use crate::{
    builder::builder::{Builder, MetaBuilder},
    lang::{builder_tree::BuilderTree, parse_main_line::LineResult, state::State},
};

use super::{InnerLangParser, Pairs, Rule};

pub(crate) fn build<MB, B>(
    state: &State,
    tree: &mut BuilderTree<B>,
    r: LineResult,
    meta_builder: &MB,
    start_index: usize,
) -> Result<(), anyhow::Error>
where
    MB: MetaBuilder<Item = B>,
    B: Builder,
{
    let (def, _emp) = state.default_types();

    let (name, arg) = if def.is_empty() {
        let mut p = InnerLangParser::parse(Rule::content, &r.content)
            //anything other than preceding space is accepted in this grammar
            .map_err(|_| anyhow!("preceding space is not allowed in this context"))?;
        parse_content(p.next().unwrap().into_inner())?
    } else {
        let full = format!("{def} {}", r.content);
        let mut p = InnerLangParser::parse(Rule::content, &full)
            //anything other than preceding space is accepted in this grammar
            .map_err(|_| anyhow!("preceding space is not allowed in this context"))?;
        parse_content(p.next().unwrap().into_inner())?
    };

    let mut builder = meta_builder.build(name, arg).map_err(|e| Error::msg(e))?;

    let params = r.params;

    for param in params {
        let p = InnerLangParser::parse(Rule::param, &param).expect("unreachable");

        let (name, val) = parse_param(p)?;
        builder.set_param(name, val).map_err(|s| Error::msg(s))?;
    }

    tree.add(builder, state.indent_level(), start_index)
        .expect("unreachable");
    Ok(())
}

pub(crate) fn build_empty_line_item<MB, B>(
    state: &State,
    tree: &mut BuilderTree<B>,
    meta_builder: &MB,
    start_index: usize,
) -> Result<(), Error>
where
    MB: MetaBuilder<Item = B>,
{
    let (_def, emp) = state.default_types();

    if emp.is_empty() {
        return Ok(());
    }

    //let emp_command = build_empty_line_command(emp, state.indent_level());

    let mut p = InnerLangParser::parse(Rule::content, &emp)
        .map_err(|_| anyhow!("preceding space is not allowed in this context"))?;
    let (name, arg) = parse_content(p.next().unwrap().into_inner())?;

    let builder = meta_builder.build(name, arg).map_err(|s| Error::msg(s))?;

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

fn parse_content(pairs: Pairs) -> Result<(String, String), Error> {
    let mut name = String::new();
    let mut text = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::name => name = pair.as_str().to_string(),
            Rule::text => text = pair.as_str().to_string(),
            Rule::EOI => {}
            _ => {
                return Err(anyhow!("{:#?}", pair));
            }
        }
    }
    Ok((name, text))
}

fn parse_param(mut pairs: Pairs) -> Result<(String, String), Error> {
    parse_content(pairs.next().unwrap().into_inner())
}

#[cfg(test)]
mod test {
    #[test]
    fn hoge() {}
}
