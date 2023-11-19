use pest::Parser;
use pest_derive::Parser;
use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::error::parse_fail::ParseFail;

/// A sample for implementing custom serde::ser::Serialize & serde::de::Deserialize for Munyo
#[derive(PartialEq, Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl serde::ser::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Munyo only uses string, so serialize_str is the way to go
        serializer.serialize_str(&format!("#{}_{}_{}", self.r, self.g, self.b))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // To deserialize a string, use type inference for String.
        let s: String = Deserialize::deserialize(deserializer)?;

        // Use serde::de::Error::custom() to report errors in Deserialize.
        // When you implement Serialize, use serde::ser::Error::custom().
        parse_color(&s).map_err(|e| serde::de::Error::custom(e))
    }
}
#[derive(Parser)]
#[grammar_inline = r###"
number_char = {
	'0'..'9'
}

number1 = @{
	number_char+
}
number2 = @{
	number_char+
}
number3 = @{
	number_char+
}

color ={
	SOI ~ "#" ~ number1 ~ "_" ~ number2 ~ "_" ~ number3 ~ EOI
}

"###]
struct ColorParser;

fn parse_color(input: &str) -> Result<Color, String> {
    let mut pairs = ColorParser::parse(Rule::color, input).map_err(|e| e.to_string())?;
    // if the grammar matched, there are always inner pairs, so you can just unwrap() and into_inner().
    let pairs = pairs.next().unwrap().into_inner();

    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    for pair in pairs {
        match pair.as_rule() {
            // When the grammar matched, number1/2/3 always exist.
            Rule::number1 => r = parse_u8(pair.as_str())?,
            Rule::number2 => g = parse_u8(pair.as_str())?,
            Rule::number3 => b = parse_u8(pair.as_str())?,
            _ => {}
        }
    }
    Ok(Color::new(r, g, b))
}

fn parse_u8(s: &str) -> Result<u8, String> {
    Ok(s.parse::<u8>().map_err(|e| e.to_string())?)
}
