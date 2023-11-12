use munyo_derive::MunyoSerialize;
use serde::{Deserialize, Serialize};

#[derive(MunyoSerialize)]
struct Sample1 {
    #[munyo(arg = 0)]
    u1: usize,
    #[munyo(arg = 1)]
    s: String,
}

#[derive(Serialize, Deserialize)]
enum Enu2 {
    It1(Sample1),
}

impl serde::ser::Serialize for Sample1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
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
