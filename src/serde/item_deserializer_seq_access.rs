use serde::de::SeqAccess;

use crate::error::parse_fail::ParseFail;

use super::item_deserializer::ItemDeserializer;

impl<'a, 'de> SeqAccess<'de> for ItemDeserializer<'a, 'de>{
    type Error = ParseFail;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {
        seed.deserialize(&mut *self).map(|r| Some(r))
    }
}