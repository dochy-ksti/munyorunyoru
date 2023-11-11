use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{samples::color::Color, Error, RestOf};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum SampleEnum {
    Item1(Color, SampleObj, SampleObj2, Vec<SampleEnum>),
    Item2(usize),
    Item3(Vec<SampleEnum>),
    Item4(SampleObj2),
    Item5(usize, RestOf),
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
    nom: Option<usize>,
}

impl SampleObj2 {
    pub(crate) fn new(nom: Option<usize>) -> Self {
        Self { nom }
    }
}

#[test]
fn des() -> Result<(), Error> {
    let o = vec![
        SampleEnum::Item1(
            Color::new(10, 20, 30),
            SampleObj::new(4, "masa".to_string()),
            SampleObj2::new(Some(5)),
            vec![
                SampleEnum::Item2(8),
                SampleEnum::Item3(vec![SampleEnum::Item4(SampleObj2::new(None))]),
            ],
        ),
        SampleEnum::Item5(30, RestOf::new("   hoge\n".to_string())),
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
