use pest::Parser;

use crate::{
    builder::builder::{Builder, MetaBuilderArguments},
    lang::{item_tree::ItemTree, parse_main_line::LineResult, state::State},
};

use super::{InnerLangParser, Pairs, Rule};

pub(crate) fn build<MB, B, T>(
    state: &mut State,
    tree: &mut ItemTree<B>,
    r: LineResult,
    meta_builder: MB,
) where
    MB: Fn(MetaBuilderArguments) -> B,
    B: Builder<T>,
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

    let mut builder = meta_builder(MetaBuilderArguments::new(name, arg));

    let params = r.params;

    for param in params {
        let p = InnerLangParser::parse(Rule::param, &param).expect("unreachable");

        let (name, val) = parse_param(p);
        builder.set_param(name, val);
    }

    tree.add(builder, state.indent_level())
        .expect("unreachable");
}

pub(crate) fn build_empty_line_item<MB, B, T>(
    state: &mut State,
    tree: &mut ItemTree<B>,
    meta_builder: MB,
) where
    MB: Fn(MetaBuilderArguments) -> B,
    B: Builder<T>,
{
    let (_def, emp) = state.default_types();

    if emp.is_empty() {
        return;
    }

    let emp_command: String = std::iter::repeat('\t')
        .take(state.indent_level())
        .chain(emp.chars())
        .collect();

    let p = InnerLangParser::parse(Rule::content, &emp_command).expect("unreachable");
    let (name, arg) = parse_content(p);

    let builder = meta_builder(MetaBuilderArguments::new(name, arg));

    tree.add(builder, state.indent_level())
        .expect("unreachable");
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
