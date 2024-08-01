use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

use crate::error::DeserializeFail;

use super::{
    ailment_change::{parse_ailment_change, AilmentChange},
    move_property::MovePropertySyntax,
    status_change::{parse_status_change, StatusChange},
};

#[derive(Parser)]
#[grammar_inline = r###"
status_ailment = {
	"やけど" | "ひるみ" 
}

sign = {
	"+" | "-"
}

number_char = _{
	'0'..'9'
}

signed_num = {
	sign ~ number_char
}

number = {
	number_char+
}
	
status_change_alpha = {
	"A" | "B" | "C" | "D" | "S" |
	"JA" | "JB" | "JC" | "JD" | "JS"
}


bracketed_percent = {
	"(" ~ percent ~ ")"
}

percent = {
	number ~ "%"
}

status_change = {
	status_change_alpha ~ signed_num ~ bracketed_percent?
}

ailment_change = {
	status_ailment ~ bracketed_percent?
}

pp = {
	"PP" ~ number
}

property = { "音" }

chunk ={
	SOI ~ (percent | status_change | ailment_change | pp | property) ~ EOI
}

"###]
struct BasicMoveChunkParser;

#[derive(Debug)]
pub(crate) enum BasicMoveChunk {
    Accuracy { percent: u32 },
    StatusChange(StatusChange),
    AilmentChange(AilmentChange),
    Property(MovePropertySyntax),
    PP(u32),
}

pub(crate) fn parse_basic_move_chunk(input: &str) -> Result<BasicMoveChunk, DeserializeFail> {
    let mut pairs = BasicMoveChunkParser::parse(Rule::chunk, input).map_err(|e| e.to_string())?;

    // if the grammar matched, there are always inner pairs, so you can just unwrap() and into_inner().
    let pair = pairs.next().unwrap().into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::percent => {
            let percent = pair.into_inner().next().unwrap().as_str().parse()?;
            if 100 < percent {
                Err("percent can't be over 100")?;
            }
            Ok(BasicMoveChunk::Accuracy { percent })
        }
        Rule::status_change => Ok(BasicMoveChunk::StatusChange(parse_status_change(
            pair.into_inner(),
        )?)),
        Rule::ailment_change => Ok(BasicMoveChunk::AilmentChange(parse_ailment_change(
            pair.into_inner(),
        )?)),
        Rule::property => Ok(BasicMoveChunk::Property(MovePropertySyntax::from_str(
            pair.as_str(),
        )?)),
        Rule::pp => {
            let pp: u32 = pair.into_inner().next().unwrap().as_str().parse()?;
            Ok(BasicMoveChunk::PP(pp))
        }
        _ => {
            unreachable!()
        }
    }
}
