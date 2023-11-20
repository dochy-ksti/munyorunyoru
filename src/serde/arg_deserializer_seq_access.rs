use serde::de::{MapAccess, SeqAccess};

use crate::error::{deserialize_error::DeserializeError, parse_fail::ParseFail};

use super::arg_deserializer::ArgDeserializer;

impl<'a, 'b, 'de> SeqAccess<'de> for &'b mut ArgDeserializer<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(|r| Some(r))
    }
}
