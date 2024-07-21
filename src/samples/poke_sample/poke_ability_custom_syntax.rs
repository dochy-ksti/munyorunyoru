#![allow(missing_docs)]

use std::str::FromStr;

use serde::de::Deserialize;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct OptPokeAbility {
    pub opt_ability: Option<PokeAbility>,
}

#[derive(PartialEq, Debug, Clone, Copy, strum::EnumString)]
pub enum PokeAbility {
    Protosynthesis,
}

impl<'de> Deserialize<'de> for OptPokeAbility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // To deserialize a string, use type inference for String.
        let s: String = Deserialize::deserialize(deserializer)?;

        let opt_ability = if s.is_empty() {
            None
        } else {
            Some(
                PokeAbility::from_str(&s)
                    .map_err(|_s| serde::de::Error::custom(format!("Ability {s} is not found")))?,
            )
        };
        Ok(OptPokeAbility { opt_ability })
    }
}
