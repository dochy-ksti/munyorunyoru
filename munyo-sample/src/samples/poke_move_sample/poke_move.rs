#[derive(Debug, Clone, Copy, serde::Deserialize, strum::EnumString)]
pub(crate) enum PokeMove {
    オーバーヒート,
    あくのはどう,
    かえんほうしゃ,
    バークアウト,
}
