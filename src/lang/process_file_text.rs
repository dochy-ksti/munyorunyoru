use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::{
        line_col_lookup::LineColLookup,
        parse_error::ParseError,
        parse_fail::{PairHelper, ParseFail},
    },
    lang::{builder_tree::BuilderTree, inner_lang::build_empty_line_item},
};

use super::{
    munyo_parser::{MunyoParser, Pairs, Rule},
    parse_line_contents::parse_line_contents,
    state::State,
};

use crate::error::parse_fail::ParseFailHelper;
use pest::Parser;

pub fn process_file_text<MB, B, T>(text: String, meta_builder: &MB) -> Result<Vec<T>, ParseError>
where
    MB: MetaBuilder<Item = B>,
    B: Builder<Item = T>,
{
    in_process_file_text(&text, meta_builder).map_err(|e| {
        let lookup = LineColLookup::new(&text);
        let r = lookup.line_col(e.start_index).unwrap();
        ParseError::new(
            r.line,
            r.col,
            text[r.line_start..r.line_end].to_string(),
            e.message,
        )
    })
}

fn in_process_file_text<MB, B, T>(text: &str, meta_builder: &MB) -> Result<Vec<T>, ParseFail>
where
    MB: MetaBuilder<Item = B>,
    B: Builder<Item = T>,
{
    let mut pairs =
        MunyoParser::parse(Rule::file, text).map_err(ParseFail::from_pest_err)?;

    let pair = pairs.next().unwrap();

    let tree = parse_file(pair.into_inner(), meta_builder)?;
    tree.finish()
}

fn parse_file<MB, B>(mut pairs: Pairs, meta_builder: &MB) -> Result<BuilderTree<B>, ParseFail>
where
    MB: MetaBuilder<Item = B>,
    B: Builder,
{
    let mut state = State::new();
    let mut tree = BuilderTree::new();

    loop {
        let tabs = pairs.next().unwrap();
        let indent_level = tabs.as_str().len();

        let p = pairs.next().unwrap();

        match p.as_rule() {
            Rule::line_contents => {
                parse_line_contents(
                    p.into_inner().next().unwrap(),
                    indent_level,
                    &mut state,
                    &mut tree,
                    meta_builder,
                )?;
                let new_line_or_eoi = pairs.next().unwrap();
                if new_line_or_eoi.as_rule() == Rule::EOI {
                    return Ok(tree);
                }
            }
            Rule::new_line => {
                build_empty_line_item(&state, &mut tree, meta_builder, p.start_index())
                    .op(&p)?;
            }
            Rule::EOI => return Ok(tree),
            _ => unreachable!(),
        }
    }
}

// pub(crate) fn parse_new_line(pair: Pair) -> String {
//     pair.as_str().to_string()
// }
