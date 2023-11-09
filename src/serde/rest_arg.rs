use serde::de::Visitor;

/// This implements custom serde::de::Deserialize, 
/// and deserialize the rest of the arguments to a String, ignoring whitespaces.
/// You need to use this as the last argument, otherwise deserialization will fail.
pub struct RestOf{
    pub arg : String
}

impl serde::ser::Serialize for RestOf{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.arg)
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor{
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting ignored_any")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(v)
    }
}

impl<'de> serde::de::Deserialize<'de> for RestOf{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            //use hidden function. This consumes the rest of the arguments.
        deserializer.deserialize_ignored_any(StrVisitor).map(|s| RestOf { arg: s })
    }
}