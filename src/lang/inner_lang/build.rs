use pest::Parser;

use crate::{builder::builder::{MetaBuilder, Builder}, lang::{state::State, parse_main_line::LineResult}};

use super::{InnerLangParser, Rule};

pub(crate) fn build<T, MB, B>(state : &State, r : LineResult, builder : MB)
where MB : MetaBuilder<B, T>, B : Builder<T>{
	let default_type = state.default_type();

	InnerLangParser::parse(Rule::content, )
}