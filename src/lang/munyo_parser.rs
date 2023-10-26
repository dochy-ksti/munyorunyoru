use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "munyo_grammar.pest"]
pub(crate) struct MunyoParser;

pub(crate) type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
pub(crate) type Pair<'a> = pest::iterators::Pair<'a, Rule>;
