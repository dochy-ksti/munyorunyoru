use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;

use crate::error::DeserializeFail;

use super::{data_types::{AilmentChange, StatAilment, StatType, StatusChange}, move_property::MovePropertySyntax};


#[derive(Parser)]
#[grammar_inline = r###"
status_ailment = {
	"Burn" | "Flinch" 
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

property = { "Sound" }

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



#[derive(strum::EnumString)]
enum StatusAlpha {
    A,
    B,
    C,
    D,
    S,
    JA,
    JB,
    JC,
    JD,
    JS,
}

pub(crate) fn parse_status_change(
    mut pairs: pest::iterators::Pairs<Rule>,
) -> Result<StatusChange, DeserializeFail> {
    let alpha = StatusAlpha::from_str(pairs.next().unwrap().as_str()).unwrap();
    let signed_num: i32 = pairs.next().unwrap().as_str().parse()?;
    let percent: u32 = parse_bracketed_percent(pairs)?;

    let (is_jibun, stat_type) = match alpha {
        StatusAlpha::A => (false, StatType::A),
        StatusAlpha::B => (false, StatType::B),
        StatusAlpha::C => (false, StatType::C),
        StatusAlpha::D => (false, StatType::D),
        StatusAlpha::S => (false, StatType::S),
        StatusAlpha::JA => (true, StatType::A),
        StatusAlpha::JB => (true, StatType::B),
        StatusAlpha::JC => (true, StatType::C),
        StatusAlpha::JD => (true, StatType::D),
        StatusAlpha::JS => (true, StatType::S),
    };

    if signed_num < -6 {
        Err("the num can't be less than -6")?
    }
    if 6 < signed_num {
        Err("the num can't be more than 6")?
    }
    Ok(StatusChange {
        is_jibun,
        stat_type,
        signed_num,
        percent,
    })
}

pub(crate) fn parse_bracketed_percent(
    mut pairs: pest::iterators::Pairs<Rule>,
) -> Result<u32, DeserializeFail> {
    if let Some(s) = pairs
        .next()
        .map(|p| p.into_inner().next().unwrap().into_inner().next().unwrap().as_str())
    {
        let num: u32 = s.parse()?;
        if 100 < num {
            Err("Percent can't be over 100")?
        }
        Ok(num)
    } else {
        Ok(100)
    }
}

pub(crate) fn parse_ailment_change(
    mut pairs: pest::iterators::Pairs<Rule>,
) -> Result<AilmentChange, DeserializeFail> {
    let stat_ailment: StatAilment = pairs.next().unwrap().as_str().parse()?;
    let percent = parse_bracketed_percent(pairs)?;
    Ok(AilmentChange {
        stat_ailment,
        percent,
    })
}
