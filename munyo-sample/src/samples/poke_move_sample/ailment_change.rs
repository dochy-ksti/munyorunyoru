use crate::{error::DeserializeFail};

use super::{basic_move_chunk_syntax::Rule, stat_ailment::StatAilment, status_change::parse_bracketed_percent};

#[derive(Debug)]
pub(crate) struct AilmentChange {
    pub(crate) stat_ailment: StatAilment,
    pub(crate) percent: u32,
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
