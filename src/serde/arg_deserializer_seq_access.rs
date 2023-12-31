use serde::de::SeqAccess;

use crate::error::deserialize_error::DeserializeError;

use super::arg_deserializer::ArgDeserializer;

impl<'a, 'b, 'de> SeqAccess<'de> for &'b mut ArgDeserializer<'a, 'de> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut **self).map(Some)
    }
}
