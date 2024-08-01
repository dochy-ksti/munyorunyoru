#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Deserialize, strum::EnumString)]
pub(crate) enum PokeType {
    #[default]
    ノ,
    炎,
    水,
    草,
    電,
    氷,
    格,
    毒,
    地,
    飛,
    エ,
    虫,
    岩,
    霊,
    竜,
    悪,
    鋼,
    妖,
    ///ステラタイプ
    ス,
    ///タイプなし。タイプが消えると全て等倍になる
    無,
}
