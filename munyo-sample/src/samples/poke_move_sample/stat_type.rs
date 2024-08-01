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
