use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::{parse_err, ParseError, ParseErrorHelper},
};

use super::{
    munyo_parser::{Pair, Pairs, Rule},
    parse_content::parse_content,
    state::State, parse_main_line::parse_main_line,
};

pub(crate) fn parse_line_contents<MB, B, T>(
    pair: Pair,
    indent_level: usize,
    state: &mut State,
    builder: &MB,
) -> Result<(), ParseError>
where
    MB: MetaBuilder<B, T>,
    B: Builder<T>,
{
    match pair.as_rule() {
        Rule::define_stmt =>{
			state.set_indent(indent_level).oe(&pair)?;
			parse_define_stmt(pair.into_inner(), indent_level, state)?
		}
        Rule::main_line => {
			state.set_indent(indent_level).oe(&pair)?;
			let r = parse_main_line(pair.into_inner())?;
		}
        Rule::commented_line => {}
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

fn parse_define_stmt(
    mut pairs: Pairs,
    indent_level: usize,
    state: &mut State,
) -> Result<(), ParseError> {
    let mut default_type: String = String::new();
    let mut empty_line_type: String = String::new();
    let mut is_doubled = false;

    state
        .set_indent(indent_level)
        .map_err(|s| parse_err(&pairs.next().unwrap(), &s))?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::define_stmt_start_symbol => match pair.as_str() {
                ">>>" => Err(parse_err(&pair, ">>> is reserved and currently unusable."))?,
                ">>" => is_doubled = true,
                ">" => is_doubled = false,
                _ => unreachable!(),
            },
            Rule::content => {
                default_type = parse_content(pair.into_inner(), "")?;
            }
            Rule::content_for_empty_line => {
                empty_line_type =
                    parse_content(pair.into_inner().next().unwrap().into_inner(), "")?;
            }
            _ => unreachable!(),
        }
    }

    if is_doubled {
        state.set_doubled_default_types(indent_level, default_type, empty_line_type)
    } else {
        state.set_single_default_types(indent_level, default_type, empty_line_type)
    }
    Ok(())
}
