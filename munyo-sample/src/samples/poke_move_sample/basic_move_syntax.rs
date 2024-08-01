#![allow(dead_code)]

use serde::Deserialize;

use crate::{
    error::DeserializeFail,
    samples::poke_move_sample::{
        ailment_change::AilmentChange,
        basic_move::{top_to_basic_move_special, BasicMove},
        basic_move_chunk_syntax::BasicMoveChunk,
        move_property::MoveProperty,
        status_change::StatusChange,
    },
};

use super::{
    basic_move_chunk_syntax::parse_basic_move_chunk, poke_move::PokeMove, poke_type::PokeType,
};

#[test]
fn test() -> munyo::Result<()> {
    let r: Vec<Top> = munyo::from_file("src/samples/poke_move_sample/basic_move_t.munyo")?;
    let r: Vec<BasicMove> = r.into_iter().map(top_to_basic_move_special).collect();
    println!("{:?}", r);

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
pub(crate) enum Top {
    Move(PokeMove, PokeType, u32, BasicMoveSyntax),
}

#[derive(Debug)]
/// ダメージを与え、追加効果をいくつか持つだけの、特殊な処理を必要としない基本的な技
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
        let s: String = deserializer.deserialize_ignored_any(munyo::IgnoredAnyVisitor)?;

        fn parse(s: &str) -> Result<BasicMoveSyntax, DeserializeFail> {
            let mut iter = s.split(' ');

            let mut accuracy: u32 = 100;
            let mut pp: Option<u32> = None;
            let mut status_changes: Vec<StatusChange> = vec![];
            let mut ailment_changes: Vec<AilmentChange> = vec![];
            let mut prop = MoveProperty::empty();

            while let Some(v) = iter.next() {
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

            let r = BasicMoveSyntax {
                accuracy,
                pp: pp.ok_or("Couldn't find 'PP'")?,
                status_changes,
                ailment_changes,
                properties: prop,
            };

            Ok(r)
        }

        parse(&s).map_err(|e| serde::de::Error::custom(e.msg()))
    }
}
