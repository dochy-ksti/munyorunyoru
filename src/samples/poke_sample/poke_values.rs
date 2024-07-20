#![allow(missing_docs)]

use std::str::FromStr;

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use serde::de::Deserialize;

#[derive(PartialEq, Debug, Clone)]
pub struct PokeValues {
    pub h: u8, 
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub s: u8,
    pub nature: Option<Nature>,
}

impl PokeValues {
    fn is_510_or_less(&self) -> bool {
        fn calc_doryokuchi(a: u8) -> u32 {
            if a <= 15 {
                0
            } else {
                if a == 16 {
                    4
                } else {
                    ((a - 16) * 8 + 4) as u32
                }
            }
        }

        let mut sum = 0;
        sum += calc_doryokuchi(self.h);
        sum += calc_doryokuchi(self.a);
        sum += calc_doryokuchi(self.b);
        sum += calc_doryokuchi(self.c);
        sum += calc_doryokuchi(self.d);
        sum += calc_doryokuchi(self.s);
        sum <= 510
    }
}

impl Default for PokeValues {
    fn default() -> Self {
        Self {
            h: 15,
            a: 15,
            b: 15,
            c: 15,
            d: 15,
            s: 15,
            nature: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Default, strum::EnumString)]
pub enum Stat {
    #[default]
    H,
    A,
    B,
    C,
    D,
    S,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Nature {
    pub up: Stat,
    pub down: Stat,
}

impl<'de> Deserialize<'de> for PokeValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // To deserialize a string, use type inference for String.
        let s: String = Deserialize::deserialize(deserializer)?;

        // Use serde::de::Error::custom() to report errors in Deserialize.
        // When you implement Serialize, use serde::*ser*::Error::custom().
        parse_doryokuchi(&s).map_err(serde::de::Error::custom)
    }
}

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
struct PokeCustomParser;

fn parse_doryokuchi(input: &str) -> Result<PokeValues, String> {
    let mut r: PokeValues = Default::default();
    let mut up: Option<Stat> = None;
    let mut down: Option<Stat> = None;
    let mut pairs = PokeCustomParser::parse(Rule::poke_custom, input).map_err(|e| e.to_string())?;
    // if the grammar matched, there are always inner pairs, so you can just unwrap() and into_inner().
    let pairs = pairs.next().unwrap().into_inner();

    for pair in pairs {
        match pair.as_rule() {
            Rule::chunk => {
                let c = parse_chunk(pair.into_inner())?;
                match c.stat {
                    Stat::H => r.h = c.value,
                    Stat::A => r.a = c.value,
                    Stat::B => r.b = c.value,
                    Stat::C => r.c = c.value,
                    Stat::D => r.d = c.value,
                    Stat::S => r.s = c.value,
                }
                if 0 < c.updown {
                    up = Some(c.stat);
                }
                if c.updown < 0 {
                    down = Some(c.stat);
                }
            }
            _ => {}
        }
    }
    if up != None && down != None {
        r.nature = Some(Nature {
            up: up.unwrap(),
            down: down.unwrap(),
        });
    } else if up == None && down == None {
        r.nature = None
    } else {
        Err("'+' and '-' is not valid")?
    }
    if r.is_510_or_less() {
        Ok(r)
    } else {
        Err("It's over 510")?
    }
}

#[derive(Default)]
struct ParseChunkResult {
    stat: Stat,
    value: u8,
    /// up 1 down -1 unchanged 0
    updown: i8,
}

fn parse_chunk(pairs: Pairs<Rule>) -> Result<ParseChunkResult, String> {
    let mut r = ParseChunkResult::default();
    for pair in pairs {
        match pair.as_rule() {
            Rule::alpha => r.stat = Stat::from_str(pair.as_str()).unwrap(),
            Rule::sign => {
                r.updown = match pair.as_str() {
                    "+" => 1,
                    "-" => -1,
                    _ => unreachable!(),
                }
            }
            Rule::number => {
                let doryokuchi = u32::from_str(pair.as_str())
                    .map_err(|_| format!("{} is not a valid number", pair.as_str()))?;
                r.value = conv_doryokuchi(doryokuchi)? + 15;
            }
            Rule::bracketed_number => {
                let pair = pair.into_inner().next().unwrap();
                let val = u32::from_str(pair.as_str())
                    .map_err(|_| format!("{} is not a valid number", pair.as_str()))?;
                if 15 < val {
                    Err(format!("({val}) must be under 16"))?
                }
                r.value = val as u8;
            }
            _ => unreachable!(),
        }
    }
    Ok(r)
}

fn conv_doryokuchi(a: u32) -> Result<u8, String> {
    if 4 <= a {
        let v = (a - 4) / 8;
        if 31 < v {
            Err(format!("{a} is bigger than 252"))?
        }
        Ok(1 + v as u8)
    } else {
        Ok(0)
    }
}

#[test]
fn test_parse_doryokuchi() -> crate::Result<()> {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    enum Enum {
        E(PokeValues),
    }
    let v: Vec<Enum> = crate::from_str(
        r###"
E H4A-(0)B4C252D4S+244"###,
    )?;

    println!("{:?}", v);
    Ok(())
}
