use serde::de::Visitor;

/// This implements custom serde::de::Deserialize,
/// and deserialize the rest of the arguments to a String, ignoring whitespaces.
/// You need to use this as the last argument, otherwise deserialization will fail.
#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RestOf {
    /// The captured string
    pub arg: String,
}

impl RestOf {
    /// Creates RestOf
    pub fn new(arg: String) -> Self {
        Self { arg }
    }
}

impl serde::ser::Serialize for RestOf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.arg)
    }
}

/// When you create a custom parser, if you need to get the rest of the line like RestOf, You can do that with this Visitor.
/// ```
/// impl<'de> serde::Deserialize<'de> for CustomData {
/// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
/// 	where D: serde::Deserializer<'de>,
/// 	{
/// 		// Get the rest of the line
/// 		let s: String = deserializer.deserialize_ignored_any(munyo::IgnoredAnyVisitor)?;
/// # 		Ok(CustomData{ s })
/// # 	}
/// # }
/// # struct CustomData{ s : String }
/// ```
pub struct IgnoredAnyVisitor;

impl<'de> Visitor<'de> for IgnoredAnyVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting ignored_any")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    // The document says visit_string without visit_str is not correct.
    // I don't know why.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for RestOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        //use hidden function. This consumes the rest of the arguments.
        deserializer
            .deserialize_ignored_any(IgnoredAnyVisitor)
            .map(|s| RestOf { arg: s })
    }
}
