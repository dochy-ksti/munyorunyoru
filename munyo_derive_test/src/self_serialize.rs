use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug)]
struct Sample1 {
    u1: usize,
    s: String,
}

impl Sample1 {
    fn new(u1: usize, s: String) -> Self {
        Self { u1, s }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Enu {
    It1(Sample1),
}

impl serde::ser::Serialize for Sample1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(value))
        (self.u1 as serde::Serialize).serialize(serializer)?;
		serializer.serialize_str(&self.s)
    }
}

impl<'de> serde::de::Deserialize<'de> for Sample1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[test]
fn test() -> munyo::Result<()> {
    let mut vec = vec![Enu::It1(Sample1::new(5, "b".to_string()))];
    let s = munyo::to_string(&vec)?;
    let r: Vec<Enu> = munyo::from_str(&s)?;
    assert_eq!(vec, r);
    Ok(())
}
