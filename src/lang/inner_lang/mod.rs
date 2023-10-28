mod build;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "inner_lang.pest"]
pub(crate) struct InnerLangParser;

pub(crate) type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
pub(crate) type Pair<'a> = pest::iterators::Pair<'a, Rule>;
