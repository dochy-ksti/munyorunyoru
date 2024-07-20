#![allow(dead_code)]
use super::custom::Custom;

const POKE_CUSTOM_TEXT: &'static str = r###"
|| <- This is the syntax for comments.
|| In the competitive Pokémon world, rankings are announced once a month.
>>>Season
2024 6 || The season of June 2024
	>>>Team
	1 || The #1 ranked team
		>>>Pokemon
		Koraidon Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
		FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong|ability Protosynthesis
			|| The following are some variations of the customization of this Pokémon
			>Item
			BoostEnergy
			FocusSash
			>Terastal
			Normal
			Ground
			Water
		|| ...
	2
	||...
2024 5
	||...
"###;

#[test]
fn test() -> crate::Result<()> {
    let r: Vec<Top> = crate::from_str(POKE_CUSTOM_TEXT)?;

    println!("{:?}", r);
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
enum Top {
    Season(usize, usize, Vec<Second>),
}

struct Season {
	year : usize,
	month : usize,
	teams : Vec<Team>
}

#[derive(Debug, serde::Deserialize)]
enum Second {
    Team(usize, Vec<Third>),
}

struct Team{
	pokemons : Vec<Pokemon>
}

#[derive(Debug, serde::Deserialize)]
enum Third {
    Pokemon(
        PokeName,
        PokeType,
        PokeItem,
        Custom,
        PokeMove,
        PokeMove,
        PokeMove,
        PokeMove,
        Param,
        Vec<Fourth>,
    ),
}

struct Pokemon{
	name : PokeName,
	poke_type : PokeType,
	item : PokeItem,
	custom : Custom,
	moves : Vec<PokeMove>,
	ability : Option<Ability>,
	variations : Vec<Fourth>,
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

#[derive(Debug, serde::Deserialize)]
enum Ability {
    Protosynthesis,
}
