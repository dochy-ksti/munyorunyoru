use crate::error::parse_error::ParseError;

use super::munyo_parser::{Pair, Pairs, Rule};

pub(crate) fn parse_line_end(pair: Pair) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::normal_end => {}
        Rule::backslash_comment_end => {}
        Rule::backslash_end => {}
        Rule::single_bar => {}
        Rule::triple_bars => {}
        Rule::double_bars => {}
        _ => unreachable!(),
    }

    Ok(())
}

fn parse_normal_end(mut pairs: Pairs) -> Result<(), ParseError> {
    let new_line = pairs.next().unwrap();
    parse_new_line(new_line.into_inner().next().unwrap());
    Ok(())
}

fn parse_new_line(pair: Pair) -> Result<(), ParseError> {
    //match pair.as_str(){
    //
    //	}
    Ok(())
}
