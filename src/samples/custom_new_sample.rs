use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Enum {
    Variant(CustomStruct),
}

/// A sample of custom new()
/// This struct has completely different data structure from its representation in the Munyo language.
#[derive(PartialEq, Debug)]
struct CustomStruct {
    byte: u8,
    float: f32,
    param_struct: ParamStruct,
}

impl CustomStruct {
    fn new(byte: u8, float: f32, param_struct: ParamStruct) -> Self {
        Self {
            byte,
            float,
            param_struct,
        }
    }
}

#[derive(PartialEq, Debug)]
struct ParamStruct {
    name: String,
    color: Color,
}

impl ParamStruct {
    fn new(name: String, color: Color) -> Self {
        Self { name, color }
    }
}

#[derive(Serialize, Deserialize)]
struct StructForSyntaxAdjustment {
    byte: u8,
    color: Color,
}

impl StructForSyntaxAdjustment {
    fn new(byte: u8, color: Color) -> Self {
        Self { byte, color }
    }
}

impl<'de> serde::de::Deserialize<'de> for CustomStruct {
    /// In Munyo, this struct is represented as:
    ///
    /// CustomStruct 12.34 my_name|color #120_240_28|byte 24
    ///
    /// "float" and "name" are in the argument position, and "color" and "byte" are parameters.
    /// But actually, "float" and "byte" are fields, and "name" and "color" are in ParamStruct.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TupleVisitor;

        /// To implement custom Serialize/Deserialize, the struct should be (de)serilized as a tuple.
        impl<'de> serde::de::Visitor<'de> for TupleVisitor {
            type Value = CustomStruct;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "CustomStruct")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Deserialize in accordance with the representation in Munyo
                let float: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                let name: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                let adjust: StructForSyntaxAdjustment = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("unexpected None"))?;
                Ok(CustomStruct::new(
                    adjust.byte,
                    float,
                    ParamStruct::new(name, adjust.color),
                ))
            }
        }
        deserializer.deserialize_tuple(2, TupleVisitor)
    }
}

impl serde::ser::Serialize for CustomStruct {
    /// Serializing and Deserializing must be the same order.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        // Serialize a struct as a tuple to implement custom serialization
        let mut st = serializer.serialize_tuple(3)?;
        st.serialize_element(&self.float)?;
        st.serialize_element(&self.param_struct.name)?;
        st.serialize_element(&StructForSyntaxAdjustment::new(
            self.byte,
            self.param_struct.color.clone(),
        ))?;
        st.end()
    }
}

#[test]
fn test() -> crate::Result<()> {
    let s = CustomStruct::new(
        10,
        2.56,
        ParamStruct::new("name".to_string(), Color::new(5, 5, 5)),
    );
    let vec = vec![Enum::Variant(s)];
    let s = crate::to_string(&vec)?;
    println!("{}", &s);
    let r: Vec<Enum> = crate::from_str(&s)?;
    assert_eq!(vec, r);
    Ok(())
}
