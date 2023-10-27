use crate::{
    builder::builder::{Builder, MetaBuilder},
    error::parse_error::{ParseError, parse_err},
};

use super::{
    munyo_parser::{Pair, Pairs, Rule},
    state::State, parse_content::parse_content,
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
    match pair.as_rule(){
        Rule::define_stmt =>{},
        Rule::main_line =>{},
        Rule::commented_line =>{},
        _ =>{ unreachable!() }
    }
    Ok(())
}

fn parse_define_stmt(pairs : Pairs, indent_level : usize, state : &mut State) -> Result<(), ParseError>{
    let mut default_type : Option<String> = None;
    let mut empty_line_type : Option<String> = None;
    let mut is_doubled : Option<bool> = None;

    for pair in pairs{
        match pair.as_rule(){
            Rule::define_stmt_start_symbol => {
                match pair.as_str(){
                    ">>" => is_doubled = Some(true),
                    ">" => is_doubled = Some(false),
                    _ => unreachable!(),
                }
            },
            Rule::content =>{
                default_type = Some(parse_content(pair.into_inner(), "")?);
            },
            Rule::content_for_empty_line =>{
                empty_line_type = Some(parse_content(pair.into_inner(), "")?);
            },
            _ => unreachable!(),
        }
    }
    state
    Ok(())
}
