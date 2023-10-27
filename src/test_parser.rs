use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "proble.pest"]
pub(crate) struct ProbleParser;
