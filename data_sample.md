Let's see how to write data effeciently in Munyo. 
This is data of competitive Pokémon battle team composition. [Full Sample]()
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
	2
	||...
2024 5
	1
	||...
```
In Munyo, every line must have typename:
```
typename arg1 arg2...
```
The above line is a bit inaccurate. If you want to learn more, read [lang_spec.txt](https://github.com/dochy-ksti/munyorunyoru/blob/master/lang_spec.txt).

You can ommit the typename by setting the default typename.
```
|| Set the default typename 'Season'
>>>Season
2024 6
|| ↑ This line becomes 'Season 2024 6' without ommiting.
```
This is the corresponding Rust data structure to capture the line.
```
#[derive(Debug, serde::Deserialize)]
enum Top {
    Season(usize, usize, Vec<Second>),
}
```
The basic usage of Munyo is to use it with 'serde'. 
To parse Munyo with serde, the deserialized line must be 'enum'.
That's not very ergonomic, but there is a reason for that, which I will discuss later.
```
	Season(usize, usize, Vec<Second>)
||  ↑typename ↑2024 ↑6     ↑ the container of child lines
```
The first argument(2024) is consumed to the first 'usize'. The second(6) is to the second.
If a line have children, the last item must be Vec of the data structure to capture the child lines.

Let's parse the Munyo text:
```Rust
let r: Vec<Top> = munyo::from_str(POKE_TEXT)?;
```


Because the arguments don't have names in the data structure, you need conversion to make it a full-fledged data structure.
```Rust
    
    let r: Vec<Season> = r.into_iter().map(top_to_season).collect();
```