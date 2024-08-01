use std::str::FromStr;

use crate::{error::DeserializeFail};

use super::{basic_move_chunk_syntax::Rule, stat_type::StatType};

#[derive(Debug)]
pub(crate) struct StatusChange {
    pub(crate) is_jibun: bool,
    pub(crate) stat_type: StatType,
    /// +6 ～ -6
    pub(crate) signed_num: i32,
    pub(crate) percent: u32,
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

/// bracketed number がない場合 100%を返す。 100以上だとエラー
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
