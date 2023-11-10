use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SampleEnum {
    Item1(Color, SampleObj, SampleObj2, Vec<SampleEnum>),
    Item2(usize),
    Item3(Vec<SampleEnum>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct SampleObj {
    num: usize,
    s: String,
}

impl SampleObj {
    pub(crate) fn new(num: usize, s: String) -> Self {
        Self { num, s }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct SampleObj2 {
    nom: usize,
    s2: String,
}

impl SampleObj2 {
    pub(crate) fn new(nom: usize, s2: String) -> Self {
        Self { nom, s2 }
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct Color {
    r: usize,
    g: usize,
    b: usize,
}

impl Color {
    pub(crate) fn new() -> Self {
        Self {
            r: 10,
            g: 10,
            b: 10,
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("#{}_{}_{}", self.r, self.g, self.b))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _s: String = Deserialize::deserialize(deserializer)?;
        //analyzing the string is tiresome
        Ok(Color {
            r: 10,
            g: 10,
            b: 10,
        })
    }
}

#[test]
fn des() -> Result<(), Error> {
    let o = vec![
        SampleEnum::Item1(
            Color::new(),
            SampleObj::new(4, "masa".to_string()),
            SampleObj2::new(5, "moso".to_string()),
            vec![
                SampleEnum::Item2(8),
                SampleEnum::Item3(vec![SampleEnum::Item2(20), SampleEnum::Item2(15)]),
            ],
        ),
        SampleEnum::Item2(30),
    ]; //, SampleEnum::SampleObj(5, SampleObj{ num : 4 })];
    let mut ser = crate::MunyoSerializer::new();
    o.serialize(&mut ser)?;
    println!("{}", ser.output());
    let mut de = crate::MunyoDeserializer::new(ser.output(), None)?;
    let deserialized: Vec<SampleEnum> = Deserialize::deserialize(&mut de)?;
    let mut ser = crate::MunyoSerializer::new();
    deserialized.serialize(&mut ser)?;
    println!("{}", ser.output());

    assert_eq!(o, deserialized);
    Ok(())
}
