use std::str::FromStr;

use serde::Deserialize;

use crate::{
    error::DeserializeFail,
    samples::poke_move_sample::basic_move_chunk_syntax::{parse_basic_move_chunk, BasicMoveChunk},
};

use super::{
    ailment_change::AilmentChange, basic_move_syntax::{self, BasicMoveSyntax, Top}, move_property::MoveProperty, poke_move::PokeMove, poke_type::PokeType, status_change::StatusChange
};

#[derive(Debug)]
pub(crate) enum DamageType {
    Special,
    Physical,
}

#[derive(Debug)]
/// ダメージを与え、追加効果をいくつか持つだけの、特殊な処理を必要としない基本的な技
pub(crate) struct BasicMove {
    pub(crate) name: PokeMove,
    pub(crate) damage_type: DamageType,
    pub(crate) power: u32,
    pub(crate) accuracy: u32,
    pub(crate) poke_type: PokeType,
    pub(crate) pp: u32,
    pub(crate) status_changes: Vec<StatusChange>,
    pub(crate) ailment_changes: Vec<AilmentChange>,
    pub(crate) properties: MoveProperty,
}

pub(crate) fn top_to_basic_move_special(top: Top) -> BasicMove {
    match top {
        Top::Move(name, poke_type, power, s) => BasicMove {
            name,
            poke_type: poke_type,
            damage_type: DamageType::Special,
            power,
            accuracy: s.accuracy,
            pp: s.pp,
            status_changes: s.status_changes,
            ailment_changes: s.ailment_changes,
            properties: s.properties,
        },
    }
}
