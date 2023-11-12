use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Enu{
	It1(usize, String)
}

struct Sample1{
	#[munyo_arg = 0]
	u1 : usize,
	#[munyo_arg = 1]
	u2 : String,
}

enum Enu2{
	It1(usize, String)
}

impl serde::ser::Serialize for Enu2{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl<'de> serde::de::Deserialize<'de> for Enu2{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        todo!()
    }
}