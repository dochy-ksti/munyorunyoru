use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::{parse_err, ParseError},
    lang::{inner_lang::build_empty_line_item, builder_tree::BuilderTree},
};

use super::{
    munyo_parser::{MunyoParser, Pair, Pairs, Rule},
    parse_content::parse_content,
    parse_line_contents::parse_line_contents,
    state::State,
};

use crate::error::parse_error::ParseErrorHelper;
use pest::Parser;

pub fn process_file_text<MB, B, T>(text: String, meta_builder: &MB) -> Result<Vec<T>, ParseError>
where
    MB: MetaBuilder<Item=B>,
	B : Builder<Item=T>,
{
    let mut pairs =
        MunyoParser::parse(Rule::file, &text).map_err(|e| ParseError::from_pest_err(e))?;

    let pair = pairs.next().unwrap();

    let tree = parse_file(pair.into_inner(), meta_builder)?;
    Ok(tree.finish())
}

fn parse_file<MB, B>(mut pairs: Pairs, meta_builder: &MB) -> Result<BuilderTree<B>, ParseError>
where
    MB: MetaBuilder<Item = B>,
	B : Builder,
{
    let mut state = State::new();
    let mut tree = BuilderTree::new(meta_builder.new(String::new(), String::new()));
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
                build_empty_line_item(&mut state, &mut tree, meta_builder);
            }
            Rule::EOI => return Ok(tree),
            _ => unreachable!(),
        }
    }
}

// pub(crate) fn parse_new_line(pair: Pair) -> String {
//     pair.as_str().to_string()
// }
