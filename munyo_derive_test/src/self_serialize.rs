
struct Sample1 {
    u1: usize,
    s: String,
}

enum Enu2 {
    It1(Sample1),
}


impl serde::ser::Serialize for Enu2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> serde::de::Deserialize<'de> for Enu2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
