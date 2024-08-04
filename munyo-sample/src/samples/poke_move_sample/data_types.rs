use super::move_property::MoveProperty;

#[derive(Debug)]
pub(crate) enum DamageType {
    Special,
    Physical,
}

#[derive(Debug)]
/// Pokemon moves which inflict damage with any number of additional effects
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

#[derive(Debug, Clone, Copy, serde::Deserialize, strum::EnumString)]
pub(crate) enum PokeMove {
    Overheat,
    DarkPulse,
    FlameThrower,
    Snarl,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Deserialize, strum::EnumString)]
pub(crate) enum PokeType {
    #[default]
    Fire,
    Dark,
}

#[derive(Debug)]
pub(crate) struct StatusChange {
    pub(crate) is_jibun: bool,
    pub(crate) stat_type: StatType,
    /// +6 ～ -6
    pub(crate) signed_num: i32,
    pub(crate) percent: u32,
}

#[derive(PartialEq, Debug, Clone, Copy, Default, strum::EnumString)]
pub(crate) enum StatType {
    #[default]
    H,
    A,
    B,
    C,
    D,
    S,
}

#[derive(Debug)]
pub(crate) struct AilmentChange {
    pub(crate) stat_ailment: StatAilment,
    pub(crate) percent: u32,
}

#[derive(Debug, strum::EnumString)]
pub(crate) enum StatAilment {
    Burn,
    Flinch,
}
