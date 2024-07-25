#![allow(dead_code)]

use std::str::FromStr;
use serde::Deserialize;
use super::poke_values::PokeValues;

#[derive(PartialEq, Debug, Clone, Copy, strum::EnumString, /* serde::Deserialize */)]
enum PokeAbility {
    Protosynthesis,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct OptPokeAbility {
    opt_ability: Option<PokeAbility>,
}

impl<'de> Deserialize<'de> for OptPokeAbility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // To deserialize a string, use type inference for String.
        let s: String = Deserialize::deserialize(deserializer)?;

		// Munyo captures an empty string when there's no arguments there.
        let opt_ability = if s.is_empty() {
            None
        } else {
			Some(
				// Munyo outputs the message with the line number and the line text when it's passed through Error::custom.
                 PokeAbility::from_str(&s)
                     .map_err(|_s| serde::de::Error::custom(format!("Ability {s} is not found")))?,
            )
			// you can use serde::Deserialize too
			//Some(Deserialize::deserialize(serde::de::value::StringDeserializer::new(s))?)
        };
        Ok(OptPokeAbility { opt_ability })
    }
}


fn third_to_pokemon(third: Third) -> Pokemon {
    match third {
        Third::Pokemon(
            name,
            poke_type,
            item,
            custom,
            move1,
            move2,
            move3,
            move4,
            opt_ability, // The customized Item
            children,
        ) => {
            let mut other_items: Vec<PokeItem> = vec![];
            let mut other_terastals: Vec<PokeType> = vec![];
            for v in children {
                match v {
                    Fourth::Item(item) => other_items.push(item),
                    Fourth::Terastal(t) => other_terastals.push(t),
                }
            }

            Pokemon {
                name,
                poke_type,
                item,
                custom,
                moves: vec![move1, move2, move3, move4],
                ability: opt_ability.opt_ability,
                other_items,
                other_terastals,
            }
        }
    }
}






const POKE_TEXT: &'static str = r###"
|| <- This is the syntax for comments.
|| In the competitive Pokémon world, rankings are announced once a month.
>>>Season
2024 6 || The season of June 2024
	>>>Team
	1 || The #1 ranked team
		>>>Pokemon
		Koraidon Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
		FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong Protosynthesis
			|| The following are some variations of the customization of this Pokémon(not necessary, just for illustration purposes)
			>Item
			BoostEnergy
			FocusSash
			>Terastal
			Normal
			Ground
			Water
		|| A team contains 6 Pokémons...
	2
	||...
2024 5
	1
	||...
"###;

#[test]
fn test() -> crate::Result<()> {
    let r: Vec<Top> = crate::from_str(POKE_TEXT)?;
    let r: Vec<Season> = r.into_iter().map(top_to_season).collect();
    println!("{:?}", r);

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
enum Top {
    Season(usize, usize, Vec<Second>),
}

#[derive(Debug, serde::Deserialize)]
enum Second {
    Team(usize, Vec<Third>),
}

#[derive(Debug, serde::Deserialize)]
enum Third {
    Pokemon(
        PokeName,
        PokeType,
        PokeItem,
        PokeValues,
        PokeMove,
        PokeMove,
        PokeMove,
        PokeMove,
        OptPokeAbility,
        Vec<Fourth>,
    ),
}

#[derive(Debug, serde::Deserialize)]
enum Fourth {
    Item(PokeItem),
    Terastal(PokeType),
}

#[derive(Debug, serde::Deserialize)]
enum PokeType {
    Fire,
    Fairy,
    Normal,
    Ground,
    Water,
}

#[derive(Debug, serde::Deserialize)]
enum PokeName {
    Koraidon,
    FlutterMane,
}

#[derive(Debug, serde::Deserialize)]
enum PokeItem {
    AssaultVest,
    ChoiceSpecs,
    BoostEnergy,
    FocusSash,
}

#[derive(Debug, serde::Deserialize)]
enum PokeMove {
    FlameCharge,
    FlareBlitz,
    DrainPunch,
    Uturn,
    MoonBlast,
    ShadowBall,
    DrainingKiss,
    PerishSong,
}

#[derive(Debug, serde::Deserialize)]
struct Param {
    ability: Option<Ability>,
}

#[derive(Debug, serde::Deserialize, strum::EnumString)]
enum Ability {
    Protosynthesis,
}

#[derive(Debug)]
struct Season {
    year: usize,
    month: usize,
    teams: Vec<Team>,
}
#[derive(Debug)]
struct Team {
    rank: usize,
    pokemons: Vec<Pokemon>,
}
#[derive(Debug)]
struct Pokemon {
    name: PokeName,
    poke_type: PokeType,
    item: PokeItem,
    custom: PokeValues,
    moves: Vec<PokeMove>,
    ability: Option<PokeAbility>,
    other_items: Vec<PokeItem>,
    other_terastals: Vec<PokeType>,
}

fn convert(top: Vec<Top>) -> Vec<Season> {
    top.into_iter().map(top_to_season).collect()
}

fn top_to_season(top: Top) -> Season {
    match top {
        Top::Season(year, month, vec) => Season {
            year,
            month,
            teams: vec.into_iter().map(second_to_team).collect(),
        },
    }
}

fn second_to_team(second: Second) -> Team {
    match second {
        Second::Team(rank, vec) => Team {
            rank,
            pokemons: vec.into_iter().map(third_to_pokemon).collect(),
        },
    }
}
