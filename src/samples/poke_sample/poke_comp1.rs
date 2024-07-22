#![allow(dead_code)]
use super::poke_values::PokeValues;

const POKE_TEXT: &'static str = r###"
|| <- This is the syntax for comments.
|| In the competitive Pokémon world, rankings are announced once a month.
>>>Season
2024 6 || The season of June 2024
	>>>Team
	1 || The #1 ranked team
		>>>Pokemon
		Koraidon Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
		FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong | ability Protosynthesis 
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

#[derive(Debug)]
struct Season {
    year: usize,
    month: usize,
    teams: Vec<Team>,
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

#[derive(Debug, serde::Deserialize)]
enum Second {
    Team(usize, Vec<Third>),
}

#[derive(Debug)]
struct Team {
    rank: usize,
    pokemons: Vec<Pokemon>,
}

fn second_to_team(second: Second) -> Team {
    match second {
        Second::Team(rank, vec) => Team {
            rank,
            pokemons: vec.into_iter().map(third_to_pokemon).collect(),
        },
    }
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
        Param,
        Vec<Fourth>,
    ),
}

#[derive(Debug)]
struct Pokemon {
    name: PokeName,
    poke_type: PokeType,
    item: PokeItem,
    custom: PokeValues,
    moves: Vec<PokeMove>,
    ability: Option<Ability>,
    other_items: Vec<PokeItem>,
    other_terastals: Vec<PokeType>,
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
            param,
            variations,
        ) => {
            let mut other_items: Vec<PokeItem> = vec![];
            let mut other_terastals: Vec<PokeType> = vec![];
            for v in variations {
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
                ability: param.ability,
                other_items,
                other_terastals,
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
enum PokeName {
    Koraidon,
    FlutterMane,
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

#[derive(Debug, serde::Deserialize)]
enum Ability {
    Protosynthesis,
}

#[derive(Debug, serde::Deserialize)]
enum Fourth {
    Item(PokeItem),
    Terastal(PokeType),
}
