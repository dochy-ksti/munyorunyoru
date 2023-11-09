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

struct IgnoredAnyVisitor;

impl<'de> Visitor<'de> for IgnoredAnyVisitor{
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting ignored_any")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(v)
    }

    // The document says visit_string without visit_str is not correct.
    // I don't know why.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(v.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for RestOf{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            //use hidden function. This consumes the rest of the arguments.
        deserializer.deserialize_ignored_any(IgnoredAnyVisitor).map(|s| RestOf { arg: s })
    }
}