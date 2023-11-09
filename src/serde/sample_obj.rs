use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::error::ReadFileError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SampleEnum {
    Item1(Color, SampleObj),
    SampleObj(usize, SampleObj)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct SampleObj {
    num: usize,
}

impl SampleObj {
    pub(crate) fn 
    new(num: usize) -> Self { Self { num } }
}

#[derive(PartialEq, Debug)]
pub(crate) struct Color{
    r : usize,
    g : usize,
    b : usize
}

impl Color {
    pub(crate) fn new() -> Self { Self { r : 10, g : 10, b : 10 } }
}

impl Serialize for Color{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&format!("#{}_{}_{}", self.r, self.g, self.b))
    }
}

impl<'de> Deserialize<'de> for Color{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let _s :String = Deserialize::deserialize(deserializer)?;
            //analyzing the string is tiresome
        Ok(Color{ r : 10, g : 10, b : 10 })
    }
}

#[test]
fn des() -> Result<(), ReadFileError> {
    let o = vec![SampleEnum::Item1(Color::new(), SampleObj::new(4))];//, SampleEnum::SampleObj(5, SampleObj{ num : 4 })];
    let mut ser = crate::MunyoSerializer::new();
    o.serialize(&mut ser)?;
	println!("{}", ser.output());
    let mut de = crate::MunyoDeserializer::new(ser.output(), None)?;
    let deserialized: Vec<SampleEnum> = Deserialize::deserialize(&mut de)?;

    assert_eq!(o, deserialized);
    Ok(())
}
