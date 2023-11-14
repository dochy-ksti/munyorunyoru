use munyo::samples::color::Color;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug)]
struct Sample1 {
    u1: usize,
    s: String,
    color: Color,
}

impl Sample1 {
    fn new(u1: usize, s: String, color: Color) -> Self {
        Self { u1, s, color }
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
        use serde::ser::SerializeTuple;
        //serializer.serialize takes self, so it can't serialize multiple values in a normal way.
        //use serialize_tuple for a workaround.
        let mut st = serializer.serialize_tuple(2)?;
        st.serialize_element(&self.u1)?;
        st.serialize_element(&self.s)?;
        st.serialize_element(&self.color)?;
        st.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for Sample1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TupleVisitor;

        impl<'de> serde::de::Visitor<'de> for TupleVisitor {
            type Value = Sample1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Sample1")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let u1: usize = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                let s: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                let color: Color = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                Ok(Sample1::new(u1, s, color))
            }
        }
        deserializer.deserialize_tuple(2, TupleVisitor)
    }
}

#[test]
fn test() -> munyo::Result<()> {
    let vec = vec![Enu::It1(Sample1::new(
        5,
        "b".to_string(),
        Color::new(10, 30, 20),
    ))];
    let s = munyo::to_string(&vec)?;
    let r: Vec<Enu> = munyo::from_str(&s)?;
    assert_eq!(vec, r);
    Ok(())
}
