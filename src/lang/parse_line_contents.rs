use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_fail::{parse_fail, PairHelper, ParseFail, ParseFailHelper, ParseFailHelper2},
};

use super::{
    builder_tree::BuilderTree,
    inner_lang::build,
    munyo_parser::{Pair, Pairs, Rule},
    parse_content::parse_content,
    parse_main_line::parse_main_line,
    state::State,
};

pub(crate) fn parse_line_contents<MB, B>(
    pair: Pair,
    indent_level: usize,
    state: &mut State,
    tree: &mut BuilderTree<B>,
    meta_builder: &MB,
) -> Result<(), ParseFail>
where
    MB: MetaBuilder<Item = B>,
    B: Builder,
{
    match pair.as_rule() {
        Rule::define_stmt => {
            state.set_indent(indent_level).op(&pair)?;
            parse_define_stmt(pair.into_inner(), indent_level, state)?;
        }
        Rule::main_line => {
            state.set_indent(indent_level).op(&pair)?;
            let start_index = pair.start_index();
            let r = parse_main_line(pair.into_inner())?;
            build(state, tree, r, meta_builder, start_index).oe(start_index)?;
        }
        Rule::commented_line => {}
        _ => {}
    }
    Ok(())
}

fn parse_define_stmt(
    mut pairs: Pairs,
    indent_level: usize,
    state: &mut State,
) -> Result<(), ParseFail> {
    let mut default_type: String = String::new();
    let mut empty_line_type: String = String::new();
    let mut is_single = false;
    let mut is_double = false;
    let mut is_triple = false;

    state
        .set_indent(indent_level)
        .map_err(|s| parse_fail(&pairs.next().unwrap(), &s))?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::define_stmt_start_symbol => match pair.as_str() {
                ">>>" => is_triple = true,
                ">>" => is_double = true,
                ">" => is_single = true,
                _ => unreachable!(),
            },
            Rule::content => {
                default_type = parse_content(pair.into_inner(), "")?;
            }
            Rule::content_for_empty_line => {
                empty_line_type =
                    parse_content(pair.into_inner().next().unwrap().into_inner(), "")?;
            }
            Rule::comment_text => {}
            _ => unreachable!(),
        }
    }

    if is_single {
        state.set_single_default_types(indent_level, default_type, empty_line_type)
    } else if is_double {
        state.set_doubled_default_types(indent_level, default_type, empty_line_type)
    } else if is_triple {
        state.set_tripled_default_types(indent_level, default_type, empty_line_type)
    } else {
        unreachable!()
    }
    Ok(())
}
