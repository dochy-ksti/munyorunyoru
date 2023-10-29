mod build;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "inner_lang.pest"]
struct InnerLangParser;

type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
//type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub(crate) use build::build;
pub(crate) use build::build_empty_line_item;
