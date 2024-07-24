# Munyo
[![crates.io link](https://img.shields.io/crates/v/munyo.svg)](https://crates.io/crates/munyo)
[![Doc link](https://docs.rs/munyo/badge.svg)](https://docs.rs/munyo)
### Munyo

Munyo is a data language which aims to be the most efficient way to handwrite data. You can also see a clear error message along with the line number when an error occurs.

Let's see how to write data effeciently in Munyo. 

This is data of competitive Pokémon battle team composition. [Full Sample](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/samples/poke_sample/poke_comp1.rs)
```
|| <- This is the syntax for comments.
|| In the competitive Pokémon world, rankings are announced once a month.
>>>Season
2024 6 || The season of June 2024
	>>>Team
	1 || The #1 ranked team
		>>>Pokemon
		Koraidon Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
		FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong | ability Protosynthesis 
			|| The followings are some variations of the customization of this 
			|| Pokémon(not necessary, just for illustration purposes)
			>Item
			BoostEnergy
			FocusSash
			>Terastal
			Normal
			Ground
			Water
		|| A team contains 6 Pokémons...
	2 ||...
	|| Players ranked 200 or higher tend to publish their Pokémon compositions in their blogs voluntarily.
2024 5
	1
	||...
```
In Munyo, every line must have typename:
```
typename arg1 arg2...
```
The above line is a bit inaccurate. If you want to learn the grammar correctly, read [lang_spec.txt](https://github.com/dochy-ksti/munyorunyoru/blob/master/lang_spec.txt).

You can ommit the typename by setting the default typename:
```
|| Set the default typename 'Season'
>>>Season
2024 6
|| ↑ This line becomes 'Season 2024 6'
```
The following is the corresponding Rust data structure to capture the line.
```
#[derive(serde::Deserialize)]
enum Top {
    Season(usize, usize, Vec<Second>),
}
```
↑ The basic usage of Munyo is to use it with 'serde'. To parse Munyo with serde, the data structure to be deserialized must be 'enum'. The enum must implement 'serde::Deserialize'. This enum implements it in the 'derive' section.
```
	Season(usize, usize, Vec<Second>)
||  ↑typename ↑2024 ↑6     ↑ the container of the data for child lines
```
↑ The first argument(2024) is consumed to the first 'usize'. The second(6) is to the second.
If a line have children, the last item must be Vec the enum which captures the child lines.

Because the arguments don't have names, you need to convert them to be a decent data structure.

First, let's parse the source text of Munyo:
```Rust
let r: Vec<Top> = munyo::from_str(.../* the sample text */)?;
```
↑ You can use 'munyo::from_str' to deserialize Munyo with serde. 'Top' implements 
'serde::Deserialize', so you can use 'munyo::from_str' with type declaration 'Vec&lt;Top&gt;'.
```Rust
let r: Vec<Season> = r.into_iter().map(top_to_season).collect();

// the full-fledged data structure
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
```
You need 'match' with single branch to handle it.

I don't have much to say about the second indentation level.
```
>>>Season
2024 6 
	>>>Team
	1 || #1 ranked team
	|| ↑ Indentation means the line is a child of the line which is one less indented.
```
```Rust
#[derive(Debug, serde::Deserialize)]
enum Second {
    Team(usize, Vec<Third>),
}

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
```
This is the third level which describes Pokemon
```
		>>>Pokemon
		Koraidon Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
```
```Rust
#[derive(serde::Deserialize)]
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

#[derive(serde::Deserialize)]
enum PokeName {
    Koraidon,
    FlutterMane,
}

#[derive(serde::Deserialize)]
enum PokeType {
    Fire,
    Fairy,
    Normal,
    Ground,
    Water,
}
//...
```
The line 'Pokemon' consists of PokeName, PokeType, PokeItem, and so on. 
These items are defined as 'enum' too.

If you write items not in the enum variants, Munyo outputs error messages like
```
9: unknown variant `Koraido`, expected `Koraidon` or `FlutterMane`
            Koraido Fire AssaultVest H204A+196B4C-0D12S92 FlameCharge FlareBlitz DrainPunch Uturn
```
When error occurs, Munyo always output the line number and the line.
In this case, serde also found out the cause correctly.

Pokemon customization traditionally has the specific representation:
```
H204A+196B4C-0D12S92
```
To parse this, you need to implement the parser. Munyo can't do this for you. [My implementation of the parser](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/samples/poke_sample/poke_values.rs)
```Rust
#[derive(Parser)]
#[grammar_inline = r###"
alpha = {
	"H" | "A" | "B" | "C" |"D"| "S"
}

sign = {
	"+" | "-"
}

number_char = _{
	'0'..'9'
}

number = {
	number_char+
}

bracketed_number ={
	"(" ~ number+ ~ ")"
}

chunk = {
	alpha ~ sign? ~ (number | bracketed_number)
}

poke_custom ={
	SOI ~ chunk+ ~ EOI
}

"###]
```
When the parser implementation returns the error message, Munyo output it with the line number and the line text:
```
10: 260 is bigger than 252
        	FlutterMane Fairy ChoiceSpecs H148A-(0)B100C260D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong | ability Protosynthesis 
```
252 is the max number for the Pokemon parameter customization.

To implement customized parser and to output useful error messages are both crucial for the most efficient data language.

The goal of this language is to reduce redundancy in text data to the greatest extent possible.
On the other hand, the backing code is not the simplest, but as you can see, it's not very complex, I think.

Pokemons have abilities, but some Pokemons have only one ability. You don't need to write it down for them.

If you need optional parameters, you can use 'param'
```
typename arg1 arg2...| param_name arg
```
↑ This is the syntax of parameters in Munyo.

The data already used it.
```
FlutterMane Fairy... | ability Protosynthesis 
```
The backing code is below:
```Rust
#[derive(serde::Deserialize)]
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
        Param,  // <- Structs are for parameters
        Vec<Fourth>,
    ),
}

#[derive(serde::Deserialize)]
struct Param {
	// field names are used as param-names
    ability: Option<Ability>,
}

#[derive(serde::Deserialize)]
enum Ability {
    Protosynthesis,
}
```
The name of the struct can be anything. It doesn't affect in Munyo. In this case, the name is 'Param'.

The struct must be 'serde::Deserialize', and the field names are used as parameter names. In this case, it's 'ability'.
```
FlutterMane Fairy... | ability Protosynthesis 
                       || ↑ the field name

Koraidon Fire... 
				 || ↑ No 'ability' parameter for this Pokemon
```
It can be Option, which means it's ommittable. 'Koraidon' doesn't have the 'ability', as you can see.

It has only one omittable parameter, which means the parameter name 'ability' can be omitted.
```
FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast... Protosynthesis
|| ↑ Attach only the ability name at the last if the Pokemon need it.
```
I created the omitted versions. [Version 1](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/samples/poke_sample/poke_comp2.rs) is simple but it doesn't have line number in the error message because
the error message is returned in the conversion process, which doesn't have the information of the line number. [Version 2](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/samples/poke_sample/poke_comp3.rs) implements a simple custom data structure to output the line number. When an error is returned in a parsing process, Munyo automatically attach the line number. Check them out if you'd like.

Pokemons basically have four moves. I implemented it naïvely.
```Rust
enum Third {
    Pokemon(
        PokeName,
        PokeType,
        PokeItem,
        PokeValues,
        PokeMove, // <- four moves
        PokeMove,
        PokeMove,
        PokeMove, // <-
        Param,  
        Vec<Fourth>,
    ),
}
```
That's more robust, but If you want to make an item have multiple subitems, 
basically you need to employ child items(or make a custom parser).

The fourth indentation level is the example for it, although they are not needed for the Pokemon data.
```
		FlutterMane Fairy ChoiceSpecs H148A-(0)B100C188D4S+68 MoonBlast ShadowBall DrainingKiss PerishSong | ability Protosynthesis 
			>Item
			BoostEnergy
			FocusSash
			>Terastal
			Normal
			Ground
			Water
```
While '>>>' defines the typename on the indentation level, '>' defines the typename at the current level.
```
Foo
	>>>TripledType
	A
		>SingledType
		B
		>
		Canceled
		>SingledType2
		C
	StillAffected
		HereIsNotCurrentLevel		
	>>>Triple2
	D
```
This becomes below:
```
Foo
	>>>TripledType
	TripledType A
		>SingledType
		SingledType B
		> 
		|| ↑ Single '>' with no name means canceling the definition.
		Canceled 
		|| ↑ Canceled is the typename of this line, because there's no default typename here
		>SingledType2 || ← defines a default type again
		SingleType2 C
	TripledType StillAffected
		ThisIsNotCurrentLevel
		|| ↑ Singled definitions don't affect on cousin levels.
	>>>Triple2
	|| ↑ Tripled definition also changable and cancellable
	Triple2 D
```

```
			>Item
			BoostEnergy
			FocusSash
			>Terastal
			Normal
			Ground
			Water
```
This means the Pokemon has 2 'Item's and 3 'Terastal's as its children.

The conversion is below:
```Rust
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
                ability: param.ability,
                other_items,
                other_terastals,
            }
        }
    }
}
```
`let mut vec = vec![]` is not ellegant, but powerful.

## Motivation

The motivation is explained [here](https://github.com/dochy-ksti/munyorunyoru/blob/master/motivation.md)

## Other Materials

[API Document](https://docs.rs/munyo/latest/munyo/)

Since Munyo is a language, the API document isn't enough to use it. Various materials are available.

[Samples](https://github.com/dochy-ksti/munyorunyoru/tree/master/src/samples) 

[Language Specifications](https://github.com/dochy-ksti/munyorunyoru/blob/master/lang_spec.txt)

[What's DSL?](https://github.com/dochy-ksti/munyorunyoru/blob/master/whats_dsl.md)

## Async

This crate also contains the [concurrent](https://docs.rs/munyo/latest/munyo/struct.Concurrent.html) version of the functions to deserialize, and runtime agnostic async fn to receive the deserialized data concurrently.

## Usage

Add these to your `cargo.toml`:

```
[dependencies]
munyo = "0.5"
serde = { version = "1", features = ["derive"] }
```
## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.