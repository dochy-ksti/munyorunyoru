#![allow(dead_code)]

use serde::Deserialize;

use crate::{
    error::DeserializeFail,
    samples::poke_move_sample::{
        custom_parser::{parse_basic_move_chunk, BasicMoveChunk},
        move_property::MoveProperty,
    },
};

use super::data_types::{AilmentChange, BasicMove, DamageType, PokeMove, PokeType, StatusChange};

#[derive(Debug, serde::Deserialize)]
pub(crate) enum Top {
    Move(PokeMove, PokeType, u32, BasicMoveSyntax),
}

#[derive(Debug)]
pub(crate) struct BasicMoveSyntax {
    pub(crate) accuracy: u32,
    pub(crate) pp: u32,
    pub(crate) status_changes: Vec<StatusChange>,
    pub(crate) ailment_changes: Vec<AilmentChange>,
    pub(crate) properties: MoveProperty,
}

impl<'de> Deserialize<'de> for BasicMoveSyntax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // deserialize_ignored_any can consume the rest of the line as a String.
        // to do that, you need 'munyo::IgnoredAnyVisitor'
        let s: String = deserializer.deserialize_ignored_any(munyo::IgnoredAnyVisitor)?;

        fn parse(s: &str) -> Result<BasicMoveSyntax, DeserializeFail> {
            // Munyo doesn't accept two consecutive whitespaces,
            // but it basically accepts one trailing whitespace.
            // If you want to follow the rule, split_terminator is a desired function.
            let mut iter = s.split_terminator(' ');

            let mut accuracy: u32 = 100;
            let mut pp: Option<u32> = None;
            let mut status_changes: Vec<StatusChange> = vec![];
            let mut ailment_changes: Vec<AilmentChange> = vec![];
            let mut prop = MoveProperty::empty();

            while let Some(v) = iter.next() {
                // If you want to accept consecutive whitespaces.
                // if v.is_empty() { continue; }

                // The custom parser gets the type of the argument and the values it contains
                match parse_basic_move_chunk(v)? {
                    BasicMoveChunk::Accuracy { percent } => {
                        accuracy = percent;
                    }
                    BasicMoveChunk::PP(value) => {
                        pp = Some(value);
                    }
                    BasicMoveChunk::StatusChange(s) => status_changes.push(s),
                    BasicMoveChunk::AilmentChange(a) => ailment_changes.push(a),
                    BasicMoveChunk::Property(p) => {
                        prop = prop | p.into();
                    }
                }
            }

            // accuracy is 100% if not defined.
            // PP must be defined.
            let r = BasicMoveSyntax {
                accuracy,
                pp: pp.ok_or("Couldn't find 'PP'")?,
                status_changes,
                ailment_changes,
                properties: prop,
            };

            Ok(r)
        }

        parse(&s).map_err(|e| serde::de::Error::custom(e.error))
    }
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
