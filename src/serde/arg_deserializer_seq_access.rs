use serde::de::{MapAccess, SeqAccess};

use crate::error::parse_fail::ParseFail;

use super::arg_deserializer::ArgDeserializer;

impl<'a, 'b, 'de> SeqAccess<'de> for &'b mut ArgDeserializer<'a, 'de> {
    type Error = ParseFail;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(|r| Some(r))
    }
}

impl<'a, 'b, 'de> MapAccess<'de> for &'b mut ArgDeserializer<'a, 'de> {
    type Error = ParseFail;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        todo!()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        todo!()
    }
}
